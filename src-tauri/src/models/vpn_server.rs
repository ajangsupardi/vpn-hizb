use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnServer {
    pub id: Option<i64>,
    pub name: String,
    pub filename: String,
    pub country: String,
    pub country_code: String,
    pub operator: Option<String>,
    pub hostname: String,
    pub ip: String,
    pub port: u16,
    pub speed: u64,
    pub ping: u16,
    pub proto: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
