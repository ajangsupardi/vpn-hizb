use crate::models::VpnStatus;
use crate::services::{ip_check, openvpn};
use crate::AppState;
use std::sync::Mutex;
use tauri::State;

pub struct VpnState {
    pub start_time: Mutex<Option<std::time::Instant>>,
    pub connected_server: Mutex<Option<String>>,
    pub cached_vpn_ip: Mutex<Option<String>>,
    pub cached_real_ip: Mutex<Option<String>>,
    pub cached_real_ipv6: Mutex<Option<String>>,
    pub cached_country: Mutex<Option<String>>,
}

impl VpnState {
    pub fn new() -> Self {
        Self {
            start_time: Mutex::new(None),
            connected_server: Mutex::new(None),
            cached_vpn_ip: Mutex::new(None),
            cached_real_ip: Mutex::new(None),
            cached_real_ipv6: Mutex::new(None),
            cached_country: Mutex::new(None),
        }
    }

    pub fn clear_cache(&self) {
        *self.cached_vpn_ip.lock().unwrap() = None;
        *self.cached_real_ip.lock().unwrap() = None;
        *self.cached_real_ipv6.lock().unwrap() = None;
        *self.cached_country.lock().unwrap() = None;
    }
}

#[tauri::command]
pub async fn get_status(
    vpn_state: State<'_, VpnState>,
) -> Result<VpnStatus, String> {
    let connected = openvpn::is_vpn_running();

    if !connected {
        vpn_state.clear_cache();
        *vpn_state.start_time.lock().unwrap() = None;
        *vpn_state.connected_server.lock().unwrap() = None;

        let real_ip = openvpn::get_real_ip();
        let real_ipv6 = openvpn::get_real_ipv6();
        return Ok(VpnStatus {
            connected: false,
            ip: None,
            real_ip,
            real_ipv6,
            country: None,
            proto: None,
            uptime: None,
            data_in: None,
            data_out: None,
            server: None,
            interface: None,
            leak: false,
            ping: None,
            speed: None,
        });
    }

    let has_cache = vpn_state.cached_vpn_ip.lock().unwrap().is_some();

    if !has_cache {
        let vpn_ip = openvpn::get_vpn_ip();
        let real_ip = openvpn::get_real_ip();
        let real_ipv6 = openvpn::get_real_ipv6();
        let country = vpn_ip
            .as_ref()
            .and_then(|ip| ip_check::get_country(ip));

        *vpn_state.cached_vpn_ip.lock().unwrap() = vpn_ip;
        *vpn_state.cached_real_ip.lock().unwrap() = real_ip.clone();
        *vpn_state.cached_real_ipv6.lock().unwrap() = real_ipv6.clone();
        *vpn_state.cached_country.lock().unwrap() = country;
    }

    let vpn_ip = vpn_state.cached_vpn_ip.lock().unwrap().clone();
    let real_ip = vpn_state.cached_real_ip.lock().unwrap().clone();
    let real_ipv6 = vpn_state.cached_real_ipv6.lock().unwrap().clone();
    let country = vpn_state.cached_country.lock().unwrap().clone();
    let server = vpn_state.connected_server.lock().unwrap().clone();
    let start = vpn_state.start_time.lock().unwrap().clone();

    let (uptime, data_in, data_out) = if let Some(st) = start {
        let elapsed = st.elapsed().as_secs();
        let hours = elapsed / 3600;
        let minutes = (elapsed % 3600) / 60;
        let seconds = elapsed % 60;
        let uptime = format!("{}h {}m {}s", hours, minutes, seconds);
        let data_in = openvpn::get_data_usage("tun0", "rx");
        let data_out = openvpn::get_data_usage("tun0", "tx");
        (Some(uptime), data_in, data_out)
    } else {
        (None, None, None)
    };

    let leak = ip_check::detect_leak(
        vpn_ip.as_deref(),
        real_ip.as_deref(),
        real_ipv6.as_deref(),
    );

    let (proto, ping, speed, server_name) = if let Some(ref filename) = server {
        let parts: Vec<&str> = filename.split('.').collect();
        let name = parts.first().unwrap_or(&"").to_string();
        (Some("tcp".to_string()), None, None, Some(name))
    } else {
        (None, None, None, None)
    };

    Ok(VpnStatus {
        connected: true,
        ip: vpn_ip,
        real_ip,
        real_ipv6,
        country,
        proto,
        uptime,
        data_in,
        data_out,
        server: server_name,
        interface: Some("tun0".to_string()),
        leak,
        ping,
        speed,
    })
}

#[tauri::command]
pub async fn get_servers(state: State<'_, AppState>) -> Result<Vec<crate::models::VpnServer>, String> {
    state
        .db
        .get_servers()
        .map_err(|e| format!("Failed to get servers: {}", e))
}

#[tauri::command]
pub async fn connect(
    server: String,
    state: State<'_, AppState>,
    vpn_state: State<'_, VpnState>,
) -> Result<VpnStatus, String> {
    let server_data = state
        .db
        .get_server_by_filename(&server)
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("Server not found")?;

    vpn_state.clear_cache();

    let status = tokio::task::spawn_blocking(move || openvpn::connect(&server_data))
        .await
        .map_err(|e| format!("Task failed: {}", e))??;

    if status.connected {
        *vpn_state.start_time.lock().unwrap() = Some(std::time::Instant::now());
        *vpn_state.connected_server.lock().unwrap() = Some(server);
        *vpn_state.cached_vpn_ip.lock().unwrap() = status.ip.clone();
        *vpn_state.cached_real_ip.lock().unwrap() = status.real_ip.clone();
        *vpn_state.cached_real_ipv6.lock().unwrap() = status.real_ipv6.clone();
        *vpn_state.cached_country.lock().unwrap() = status.country.clone();
    }

    Ok(status)
}

#[tauri::command]
pub async fn disconnect(vpn_state: State<'_, VpnState>) -> Result<(), String> {
    tokio::task::spawn_blocking(openvpn::disconnect)
        .await
        .map_err(|e| format!("Task failed: {}", e))??;
    *vpn_state.start_time.lock().unwrap() = None;
    *vpn_state.connected_server.lock().unwrap() = None;
    vpn_state.clear_cache();
    Ok(())
}
