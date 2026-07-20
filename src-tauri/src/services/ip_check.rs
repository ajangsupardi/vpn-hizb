use std::process::Command;

pub fn get_country(ip: &str) -> Option<String> {
    let url = format!("http://ip-api.com/json/{}?fields=countryCode", ip);
    let output = Command::new("curl")
        .args([
            "-s",
            "--connect-timeout",
            "2",
            "--max-time",
            "5",
            &url,
        ])
        .output()
        .ok()?;

    if output.status.success() {
        let body = String::from_utf8_lossy(&output.stdout).to_string();
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(code) = parsed.get("countryCode").and_then(|v| v.as_str()) {
                return Some(code.to_string());
            }
        }
    }
    None
}

pub fn detect_leak(vpn_ip: Option<&str>, real_ip: Option<&str>, real_ipv6: Option<&str>) -> bool {
    if let (Some(vpn), Some(real)) = (vpn_ip, real_ip) {
        if vpn != real {
            return true;
        }
    }
    if let Some(ipv6) = real_ipv6 {
        if !ipv6.is_empty() {
            return true;
        }
    }
    false
}
