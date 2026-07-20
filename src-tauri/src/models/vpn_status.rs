use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStatus {
    pub connected: bool,
    pub ip: Option<String>,
    pub real_ip: Option<String>,
    pub real_ipv6: Option<String>,
    pub country: Option<String>,
    pub proto: Option<String>,
    pub uptime: Option<String>,
    pub data_in: Option<String>,
    pub data_out: Option<String>,
    pub server: Option<String>,
    pub interface: Option<String>,
    pub leak: bool,
    pub ping: Option<u16>,
    pub speed: Option<u64>,
}
