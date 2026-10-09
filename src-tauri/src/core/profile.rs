use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    WireGuard,
    OpenVpn,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileMeta {
    pub protocol: Protocol,
    pub name: Option<String>,
    pub endpoint: Option<String>,
    pub addresses: Vec<String>,
    pub dns: Vec<String>,
    pub allowed_ips: Vec<String>,
}

pub fn detect(text: &str) -> Protocol {
    let has = |needle: &str| text.to_ascii_lowercase().contains(needle);
    if has("[interface]") || has("[peer]") {
        Protocol::WireGuard
    } else if has("client") && (has("remote ") || has("<ca>")) {
        Protocol::OpenVpn
    } else if has("remote ") || has("dev tun") || has("dev tap") {
        Protocol::OpenVpn
    } else {
        Protocol::Unknown
    }
}

fn clean_lines(text: &str) -> impl Iterator<Item = String> + '_ {
    text.lines().map(|line| {
        let no_comment = line.split(['#', ';']).next().unwrap_or("");
        no_comment.split_whitespace().collect::<Vec<_>>().join(" ")
    })
}

fn parse_wireguard(text: &str) -> ProfileMeta {
    let mut meta = ProfileMeta {
        protocol: Protocol::WireGuard,
        name: None,
        endpoint: None,
        addresses: Vec::new(),
        dns: Vec::new(),
        allowed_ips: Vec::new(),
    };
    let mut section = String::new();
    for line in clean_lines(text) {
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            section = line.to_ascii_lowercase();
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_ascii_lowercase();
            let value = value.trim().to_string();
            match (section.as_str(), key.as_str()) {
                ("[interface]", "address") => {
                    meta.addresses.extend(value.split(',').map(|s| s.trim().to_string()))
                }
                ("[interface]", "dns") => {
                    meta.dns.extend(value.split(',').map(|s| s.trim().to_string()))
                }
                ("[peer]", "endpoint") => meta.endpoint = Some(value),
                ("[peer]", "allowedips") => meta
                    .allowed_ips
                    .extend(value.split(',').map(|s| s.trim().to_string())),
                _ => {}
            }
        }
    }
    meta
}

fn parse_openvpn(text: &str) -> ProfileMeta {
    let mut meta = ProfileMeta {
        protocol: Protocol::OpenVpn,
        name: None,
        endpoint: None,
        addresses: Vec::new(),
        dns: Vec::new(),
        allowed_ips: Vec::new(),
    };
    for line in clean_lines(text) {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("remote") => {
                let host = parts.next().unwrap_or("").to_string();
                let port = parts.next().unwrap_or("1194").to_string();
                if !host.is_empty() {
                    meta.endpoint = Some(format!("{host}:{port}"));
                }
            }
            Some("dev") => {
                if let Some(dev) = parts.next() {
                    meta.addresses.push(dev.to_string());
                }
            }
            _ => {}
        }
    }
    meta
}

pub fn parse(name: Option<&str>, text: &str) -> Result<ProfileMeta, String> {
    if text.trim().is_empty() {
        return Err("الملف فارغ".into());
    }
    let mut meta = match detect(text) {
        Protocol::WireGuard => parse_wireguard(text),
        Protocol::OpenVpn => parse_openvpn(text),
        Protocol::Unknown => {
            return Err("تعذّر التعرّف على نوع الملف (المتوقع .conf أو .ovpn)".into())
        }
    };
    meta.name = name.map(|n| n.to_string());
    Ok(meta)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WG: &str = r#"
[Interface]
# the interface key
PrivateKey = abc123
Address = 10.7.0.2/32, 10.7.0.3/32
DNS = 1.1.1.1

[Peer]
PublicKey = peerkey
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = 203.0.113.5:51820
"#;

    const OVPN: &str = r#"
client
dev tun
proto udp
remote vpn.example.com 1194
<ca>
-----BEGIN CERTIFICATE-----
foo
-----END CERTIFICATE-----
</ca>
"#;

    #[test]
    fn detects_wireguard() {
        assert_eq!(detect(WG), Protocol::WireGuard);
    }

    #[test]
    fn parses_wireguard_fields() {
        let meta = parse(Some("ny.conf"), WG).unwrap();
        assert_eq!(meta.protocol, Protocol::WireGuard);
        assert_eq!(meta.endpoint.as_deref(), Some("203.0.113.5:51820"));
        assert_eq!(meta.addresses, vec!["10.7.0.2/32", "10.7.0.3/32"]);
        assert_eq!(meta.dns, vec!["1.1.1.1"]);
        assert_eq!(meta.allowed_ips, vec!["0.0.0.0/0", "::/0"]);
    }

    #[test]
    fn parses_openvpn_fields() {
        let meta = parse(None, OVPN).unwrap();
        assert_eq!(meta.protocol, Protocol::OpenVpn);
        assert_eq!(meta.endpoint.as_deref(), Some("vpn.example.com:1194"));
    }

    #[test]
    fn rejects_empty_and_unknown() {
        assert!(parse(None, "   ").is_err());
        assert!(parse(None, "hello world").is_err());
    }
}
