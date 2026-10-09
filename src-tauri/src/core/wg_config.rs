use base64::Engine;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

#[derive(Debug, Clone)]
pub struct WgConfig {
    pub private_key: [u8; 32],
    pub peer_public_key: [u8; 32],
    pub preshared_key: Option<[u8; 32]>,
    pub endpoint: SocketAddr,
    pub address: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub dns: Vec<IpAddr>,
    pub allowed_ips: Vec<String>,
    pub mtu: usize,
    pub keepalive: Option<u16>,
}

fn decode_key(value: &str) -> Result<[u8; 32], String> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(value.trim())
        .map_err(|_| "مفتاح base64 غير صالح".to_string())?;
    if decoded.len() != 32 {
        return Err("طول المفتاح يجب أن يكون 32 بايت".to_string());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&decoded);
    Ok(out)
}

pub fn prefix_to_mask(prefix: u8) -> Ipv4Addr {
    let bits: u32 = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix as u32) };
    Ipv4Addr::from(bits)
}

pub fn parse(text: &str) -> Result<WgConfig, String> {
    let mut private_key: Option<[u8; 32]> = None;
    let mut public_key: Option<[u8; 32]> = None;
    let mut preshared_key: Option<[u8; 32]> = None;
    let mut address: Option<Ipv4Addr> = None;
    let mut netmask = Ipv4Addr::new(255, 255, 255, 255);
    let mut dns = Vec::new();
    let mut allowed_ips = Vec::new();
    let mut mtu = 1420usize;
    let mut keepalive = None;
    let mut endpoint_host: Option<(String, u16)> = None;
    let mut section = String::new();

    for raw in text.lines() {
        let line = raw.split(['#', ';']).next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            section = line.to_ascii_lowercase();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        match key.as_str() {
            "privatekey" => private_key = Some(decode_key(value)?),
            "presharedkey" => preshared_key = Some(decode_key(value)?),
            "publickey" => public_key = Some(decode_key(value)?),
            "address" => {
                let first = value.split(',').next().unwrap_or("").trim();
                if let Some((ip, prefix)) = first.split_once('/') {
                    if let Ok(v4) = ip.trim().parse::<Ipv4Addr>() {
                        address = Some(v4);
                        if let Ok(p) = prefix.trim().parse::<u8>() {
                            netmask = prefix_to_mask(p.min(32));
                        }
                    }
                }
            }
            "dns" => {
                for part in value.split(',') {
                    if let Ok(ip) = part.trim().parse::<IpAddr>() {
                        dns.push(ip);
                    }
                }
            }
            "mtu" => {
                if let Ok(v) = value.parse::<usize>() {
                    mtu = v;
                }
            }
            "persistentkeepalive" => keepalive = value.parse::<u16>().ok(),
            "allowedips" => {
                for part in value.split(',') {
                    let p = part.trim();
                    if !p.is_empty() {
                        allowed_ips.push(p.to_string());
                    }
                }
            }
            "endpoint" => {
                if let Some((host, port)) = value.rsplit_once(':') {
                    if let Ok(p) = port.trim().parse::<u16>() {
                        endpoint_host = Some((host.trim().to_string(), p));
                    }
                }
            }
            _ => {
                let _ = &section;
            }
        }
    }

    let private_key = private_key.ok_or("مفقود PrivateKey")?;
    let peer_public_key = public_key.ok_or("مفقود PublicKey للسيرفر")?;
    let address = address.ok_or("مفقود Address")?;
    let (host, port) = endpoint_host.ok_or("مفقود Endpoint")?;
    let endpoint = format!("{host}:{port}")
        .to_socket_addrs()
        .map_err(|_| format!("تعذّر تحليل Endpoint: {host}:{port}"))?
        .next()
        .ok_or("تعذّر تحليل Endpoint")?;

    Ok(WgConfig {
        private_key,
        peer_public_key,
        preshared_key,
        endpoint,
        address,
        netmask,
        dns,
        allowed_ips,
        mtu,
        keepalive,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key() -> String {
        base64::engine::general_purpose::STANDARD.encode([7u8; 32])
    }

    #[test]
    fn parses_full_config() {
        let text = format!(
            "[Interface]\nPrivateKey = {k}\nAddress = 10.7.0.2/24\nDNS = 1.1.1.1, 8.8.8.8\nMTU = 1380\n\n[Peer]\nPublicKey = {k}\nPresharedKey = {k}\nAllowedIPs = 0.0.0.0/0, ::/0\nEndpoint = 203.0.113.9:51820\nPersistentKeepalive = 25\n",
            k = key()
        );
        let cfg = parse(&text).unwrap();
        assert_eq!(cfg.address, Ipv4Addr::new(10, 7, 0, 2));
        assert_eq!(cfg.netmask, Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(cfg.endpoint.port(), 51820);
        assert_eq!(cfg.dns.len(), 2);
        assert_eq!(cfg.mtu, 1380);
        assert_eq!(cfg.keepalive, Some(25));
        assert_eq!(cfg.allowed_ips.len(), 2);
        assert!(cfg.preshared_key.is_some());
    }

    #[test]
    fn rejects_bad_key() {
        let text = "[Interface]\nPrivateKey = not_base64!!\nAddress = 10.0.0.1/32\n[Peer]\nPublicKey = abc\nEndpoint = 1.2.3.4:51820\n";
        assert!(parse(text).is_err());
    }

    #[test]
    fn missing_endpoint_errors() {
        let text = format!("[Interface]\nPrivateKey = {k}\nAddress = 10.0.0.1/32\n[Peer]\nPublicKey = {k}\n", k = key());
        assert!(parse(&text).is_err());
    }
}
