use crate::models::VpnServer;
use base64::Engine;
use regex::Regex;
use reqwest::blocking::Client;
use std::time::Duration;

const API_URL: &str = "http://www.vpngate.net/api/iphone/";

pub struct VpngateApiService {
    client: Client,
}

impl VpngateApiService {
    pub fn new() -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");
        Self { client }
    }

    pub fn fetch_servers(&self) -> Result<Vec<VpnServer>, String> {
        let response = self
            .client
            .get(API_URL)
            .header("Accept-Encoding", "gzip, deflate")
            .send()
            .map_err(|e| format!("Failed to fetch VPN servers: {}", e))?;

        let body = response
            .text()
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let servers = self.parse_csv(&body);
        Ok(servers)
    }

    fn parse_csv(&self, body: &str) -> Vec<VpnServer> {
        let mut servers = Vec::new();
        let config_re = Regex::new(r#"(?m)^remote\s+([\d.]+)\s+(\d+)"#).unwrap();
        let proto_re = Regex::new(r"(?m)^proto\s+(udp|tcp)").unwrap();

        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with('*')
                || trimmed.starts_with('#')
                || trimmed.contains("ServerName")
            {
                continue;
            }

            let fields: Vec<&str> = trimmed.split(',').collect();
            if fields.len() < 15 {
                continue;
            }

            if let Some(server) = self.parse_row(&fields, &config_re, &proto_re) {
                servers.push(server);
            }
        }
        servers
    }

    fn parse_row(
        &self,
        fields: &[&str],
        config_re: &Regex,
        proto_re: &Regex,
    ) -> Option<VpnServer> {
        let name = fields[0].trim().to_string();
        let ip = fields[1].trim().to_string();
        let country = fields[5].trim().to_string();
        let country_code = fields[6].trim().to_string();
        let speed: u64 = fields[4].trim().parse().unwrap_or(0);
        let ping: u16 = fields[3].trim().parse().unwrap_or(0);

        let operator_raw = fields[12].trim();
        let operator = if operator_raw.contains("'s owner") {
            let cleaned = operator_raw.replace("'s owner", "");
            let parts: Vec<&str> = cleaned.split('_').collect();
            Some(parts[0].trim().to_string())
        } else {
            let parts: Vec<&str> = operator_raw.split('_').collect();
            let val = parts[0].trim().to_string();
            if val.is_empty() {
                None
            } else {
                Some(val)
            }
        };

        let mut port: u16 = 443;
        let mut proto = "udp".to_string();

        if let Ok(config_bytes) = base64::engine::general_purpose::STANDARD.decode(fields[14].trim()) {
            if let Ok(config_str) = String::from_utf8(config_bytes) {
                if let Some(caps) = config_re.captures(&config_str) {
                    if let Some(p) = caps.get(2) {
                        port = p.as_str().parse().unwrap_or(443);
                    }
                }
                if let Some(caps) = proto_re.captures(&config_str) {
                    if let Some(p) = caps.get(1) {
                        proto = p.as_str().to_string();
                    }
                }
            }
        }

        let hostname = format!("{}:{}", ip, port);
        let filename = format!("{}.ovpn", name);

        Some(VpnServer {
            id: None,
            name,
            filename,
            country,
            country_code,
            operator,
            hostname,
            ip,
            port,
            speed,
            ping,
            proto,
            created_at: None,
            updated_at: None,
        })
    }
}
