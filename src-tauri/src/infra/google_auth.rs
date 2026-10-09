use base64::Engine;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::TcpListener;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GoogleUser {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
}

fn b64url(data: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

fn open_browser(url: &str) {
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
}

fn extract_param(request: &str, key: &str) -> Option<String> {
    let line = request.lines().next().unwrap_or("");
    let query = line.split('?').nth(1)?.split(' ').next()?;
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == key {
                return Some(urlencoding::decode(v).map(|c| c.into_owned()).unwrap_or_default());
            }
        }
    }
    None
}

const SUCCESS_PAGE: &str = "<!doctype html><html dir='rtl'><head><meta charset='utf-8'><title>RikaVPN</title></head><body style='font-family:system-ui,segie ui,sans-serif;text-align:center;padding:60px;background:#f4f5fa;color:#14162b'><h2 style='color:#6c5ce7'>تم تسجيل الدخول بنجاح</h2><p>يمكنك إغلاق هذه الصفحة والعودة إلى RikaVPN.</p></body></html>";

fn respond(stream: &mut std::net::TcpStream, body: &str) {
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.as_bytes().len(),
        body
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.flush();
}

pub fn login(client_id: &str) -> Result<GoogleUser, String> {
    if client_id.trim().is_empty() {
        return Err("GOOGLE_CLIENT_ID_MISSING".into());
    }

    let verifier = {
        use rand::Rng;
        let bytes: [u8; 32] = rand::thread_rng().gen();
        b64url(&bytes)
    };
    let challenge = b64url(&Sha256::digest(verifier.as_bytes()));

    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let redirect = format!("http://127.0.0.1:{port}");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&code_challenge={}&code_challenge_method=S256&access_type=offline&prompt=select_account",
        urlencoding::encode(client_id),
        urlencoding::encode(&redirect),
        urlencoding::encode("openid email profile"),
        challenge
    );

    tracing::info!("opening Google OAuth in browser");
    open_browser(&auth_url);

    let (mut stream, _) = listener.accept().map_err(|e| e.to_string())?;
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
    let request = String::from_utf8_lossy(&buf[..n]).to_string();

    if let Some(err) = extract_param(&request, "error") {
        respond(&mut stream, SUCCESS_PAGE);
        return Err(format!("تم رفض التفويض: {err}"));
    }
    let code = extract_param(&request, "code").ok_or("لم يتم استلام رمز التفويض")?;
    respond(&mut stream, SUCCESS_PAGE);

    let token_json: serde_json::Value = ureq::post("https://oauth2.googleapis.com/token")
        .send_form(&[
            ("client_id", client_id),
            ("code", code.as_str()),
            ("code_verifier", verifier.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect.as_str()),
        ])
        .map_err(|e| format!("فشل تبادل الرمز: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;

    let access = token_json["access_token"]
        .as_str()
        .ok_or("لم يُرجِع Google رمز الوصول")?;

    let user_json: serde_json::Value = ureq::get("https://openidconnect.googleapis.com/v1/userinfo")
        .set("Authorization", &format!("Bearer {access}"))
        .call()
        .map_err(|e| format!("فشل جلب بيانات الحساب: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;

    Ok(GoogleUser {
        sub: user_json["sub"].as_str().unwrap_or("").to_string(),
        name: user_json["name"].as_str().unwrap_or("").to_string(),
        email: user_json["email"].as_str().unwrap_or("").to_string(),
        picture: user_json["picture"].as_str().map(|s| s.to_string()),
    })
}
