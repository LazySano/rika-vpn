use serde::Serialize;
use std::net::{IpAddr, ToSocketAddrs};

#[derive(Debug, Clone, Serialize)]
pub struct GeoInfo {
    pub country_code: String,
    pub city: String,
}

pub fn lookup(host: &str) -> Result<GeoInfo, String> {
    let ip: IpAddr = match host.parse() {
        Ok(ip) => ip,
        Err(_) => host
            .to_socket_addrs()
            .map_err(|e| e.to_string())?
            .next()
            .ok_or("تعذّر تحليل عنوان السيرفر")?
            .ip(),
    };

    let url = format!("https://ipwho.is/{ip}");
    let json: serde_json::Value = ureq::get(&url)
        .call()
        .map_err(|e| e.to_string())?
        .into_json()
        .map_err(|e| e.to_string())?;

    Ok(GeoInfo {
        country_code: json["country_code"].as_str().unwrap_or("").to_string(),
        city: json["city"].as_str().unwrap_or("").to_string(),
    })
}
