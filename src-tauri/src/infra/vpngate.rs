use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VpnGateServer {
    pub hostname: String,
    pub ip: String,
    pub country_short: String,
    pub country_long: String,
    pub ping: u32,
    pub speed: u64,
    pub sessions: u32,
    pub config: String,
}

pub fn fetch(limit: usize, country: Option<String>) -> Result<Vec<VpnGateServer>, String> {
    let urls = [
        "https://r.jina.ai/http://www.vpngate.net/api/iphone/",
        "https://www.vpngate.net/api/iphone/",
    ];
    let mut body: Option<String> = None;
    let mut last_err = String::new();
    for url in urls {
        match ureq::get(url).set("User-Agent", "Mozilla/5.0").call() {
            Ok(resp) => match resp.into_string() {
                Ok(text) if text.contains("#HostName") || text.contains("OpenVPN_ConfigData") => {
                    body = Some(text);
                    break;
                }
                Ok(_) => last_err = format!("رد غير متوقّع من {url}"),
                Err(e) => last_err = e.to_string(),
            },
            Err(e) => last_err = e.to_string(),
        }
    }
    let body = body.ok_or_else(|| format!("تعذّر جلب قائمة VPNGate: {last_err}"))?;

    let filter = country.map(|c| c.to_ascii_uppercase());
    let mut out: Vec<VpnGateServer> = Vec::new();
    let mut in_data = false;

    for line in body.lines() {
        if line.starts_with('#') {
            if line.contains("HostName") {
                in_data = true;
            }
            continue;
        }
        if line.starts_with('*') {
            if in_data {
                break;
            }
            continue;
        }
        if !in_data || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(15, ',').collect();
        if parts.len() < 15 {
            continue;
        }
        let short = parts[6].to_string();
        if let Some(f) = &filter {
            if !short.eq_ignore_ascii_case(f) {
                continue;
            }
        }
        out.push(VpnGateServer {
            hostname: parts[0].to_string(),
            ip: parts[1].to_string(),
            country_short: short,
            country_long: parts[5].to_string(),
            ping: parts[3].parse().unwrap_or(0),
            speed: parts[4].parse().unwrap_or(0),
            sessions: parts[7].parse().unwrap_or(0),
            config: parts[14].to_string(),
        });
    }

    out.sort_by(|a, b| a.ping.cmp(&b.ping).then(b.speed.cmp(&a.speed)));
    out.truncate(limit);
    Ok(out)
}
