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
    let body = ureq::get("https://www.vpngate.net/api/iphone/")
        .call()
        .map_err(|e| format!("تعذّر جلب قائمة VPNGate: {e}"))?
        .into_string()
        .map_err(|e| e.to_string())?;

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
