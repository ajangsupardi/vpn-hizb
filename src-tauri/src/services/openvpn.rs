use crate::models::{VpnServer, VpnStatus};
use crate::services::ip_check;
use crate::services::platform;
use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};

const OVPN_CONFIG_PATH: &str = "/tmp/vpn-hizb-connect.ovpn";
const OVPN_AUTH_PATH: &str = "/tmp/vpn-hizb-auth.txt";
const OVPN_LOG_PATH: &str = "/tmp/vpn-hizb-openvpn.log";
const CA_CERT: &str = "-----BEGIN CERTIFICATE-----
MIIFazCCA1OgAwIBAgIRAIIQz7DSQONZRGPgu2OCiwAwDQYJKoZIhvcNAQELBQAw
TzELMAkGA1UEBhMCVVMxKTAnBgNVBAoTIEludGVybmV0IFNlY3VyaXR5IFJlc2Vh
cmNoIEdyb3VwMRUwEwYDVQQDEwxJU1JHIFJvb3QgWDEwHhcNMTUwNjA0MTEwNDM4
WhcNMzUwNjA0MTEwNDM4WjBPMQswCQYDVQQGEwJVUzEpMCcGA1UEChMgSW50ZXJu
ZXQgU2VjdXJpdHkgUmVzZWFyY2ggR3JvdXAxFTATBgNVBAMTDElTUkcgUm9vdCBY
MTCCAiIwDQYJKoZIhvcNAQEBBQADggIPADCCAgoCggIBAK3oJHP0FDfzm54rVygc
h77ct984kIxuPOZXoHj3dcKi/vVqbvYATyjb3miGbESTtrFj/RQSa78f0uoxmyF+
0TM8ukj13Xnfs7j/EvEhmkvBioZxaUpmZmyPfjxwv60pIgbz5MDmgK7iS4+3mX6U
A5/TR5d8mUgjU+g4rk8Kb4Mu0UlXjIB0ttov0DiNewNwIRt18jA8+o+u3dpjq+sW
T8KOEUt+zwvo/7V3LvSye0rgTBIlDHCNAymg4VMk7BPZ7hm/ELNKjD+Jo2FR3qyH
B5T0Y3HsLuJvW5iB4YlcNHlsdu87kGJ55tukmi8mxdAQ4Q7e2RCOFvu396j3x+UC
B5iPNgiV5+I3lg02dZ77DnKxHZu8A/lJBdiB3QW0KtZB6awBdpUKD9jf1b0SHzUv
KBds0pjBqAlkd25HN7rOrFleaJ1/ctaJxQZBKT5ZPt0m9STJEadao0xAH0ahmbWn
OlFuhjuefXKnEgV4We0+UXgVCwOPjdAvBbI+e0ocS3MFEvzG6uBQE3xDk3SzynTn
jh8BCNAw1FtxNrQHusEwMFxIt4I7mKZ9YIqioymCzLq9gwQbooMDQaHWBfEbwrbw
qHyGO0aoSCqI3Haadr8faqU9GY/rOPNk3sgrDQoo//fb4hVC1CLQJ13hef4Y53CI
rU7m2Ys6xt0nUW7/vGT1M0NPAgMBAAGjQjBAMA4GA1UdDwEB/wQEAwIBBjAPBgNV
HRMBAf8EBTADAQH/MB0GA1UdDgQWBBR5tFnme7bl5AFzgAiIyBpY9umbbjANBgkq
hkiG9w0BAQsFAAOCAgEAVR9YqbyyqFDQDLHYGmkgJykIrGF1XIpu+ILlaS/V9lZL
ubhzEFnTIZd+50xx+7LSYK05qAvqFyFWhfFQDlnrzuBZ6brJFe+GnY+EgPbk6ZGQ
3BebYhtF8GaV0nxvwuo77x/Py9auJ/GpsMiu/X1+mvoiBOv/2X/qkSsisRcOj/KK
NFtY2PwByVS5uCbMiogziUwthDyC3+6WVwW6LLv3xLfHTjuCvjHIInNzktHCgKQ5
ORAzI4JMPJ+GslWYHb4phowim57iaztXOoJwTdwJx4nLCgdNbOhdjsnvzqvHu7Ur
TkXWStAmzOVyyghqpZXjFaH3pO3JLF+l+/+sKAIuvtd7u+Nxe5AW0wdeRlN8NwdC
jNPElpzVmbUq4JUagEiuTDkHzsxHpFKVK7q4+63SM1N95R1NbdWhscdCb+ZAJzVc
oyi3B43njTOQ5yOf+1CceWxG1bQVs5ZufpsMljq4Ui0/1lvh+wjChP4kqKOJ2qxq
4RgqsahDYVvTH9w7jXbyLeiNdd8XM2w9U/t7y0Ff/9yi0GE44Za4rF2LN9d11TPA
mRGunUHBcnWEvgJBQl9nJEiU0Zsnvgc/ubhPgXRR4Xq37Z0j4r7g1SgEEzwxA57d
emyPxgcYxn/eR44/KJ4EBs+lVDR3veyJm+kXQ99b21/+jh5Xos1AnX5iItreGCc=
-----END CERTIFICATE-----";

pub fn is_vpn_running() -> bool {
    let pgrep = Command::new("pgrep")
        .args(["-x", "openvpn"])
        .output();

    let tun0 = Command::new("ip")
        .args(["addr", "show", "tun0"])
        .output();

    match (pgrep, tun0) {
        (Ok(p), Ok(t)) => {
            let pgrep_ok = p.status.success() && !p.stdout.is_empty();
            let tun0_ok = t.status.success();
            pgrep_ok && tun0_ok
        }
        _ => false,
    }
}

pub fn get_status(
    start_time: &Option<Instant>,
    connected_server: &Option<String>,
) -> VpnStatus {
    let connected = is_vpn_running();

    if connected {
        let vpn_ip = get_vpn_ip();
        let real_ip = get_real_ip();
        let real_ipv6 = get_real_ipv6();

        let country = vpn_ip
            .as_ref()
            .and_then(|ip| ip_check::get_country(ip));

        let (uptime, data_in, data_out) = if let Some(st) = start_time {
            let elapsed = st.elapsed().as_secs();
            let hours = elapsed / 3600;
            let minutes = (elapsed % 3600) / 60;
            let seconds = elapsed % 60;
            let uptime = format!("{}h {}m {}s", hours, minutes, seconds);
            let data_in = get_data_usage("tun0", "rx");
            let data_out = get_data_usage("tun0", "tx");
            (Some(uptime), data_in, data_out)
        } else {
            (None, None, None)
        };

        let leak = ip_check::detect_leak(
            vpn_ip.as_deref(),
            real_ip.as_deref(),
            real_ipv6.as_deref(),
        );

        let (proto, ping, speed, server_name) = if let Some(filename) = connected_server {
            let parts: Vec<&str> = filename.split('.').collect();
            let name = parts.first().unwrap_or(&"").to_string();
            (Some("tcp".to_string()), None, None, Some(name))
        } else {
            (None, None, None, None)
        };

        VpnStatus {
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
        }
    } else {
        let real_ip = get_real_ip();
        let real_ipv6 = get_real_ipv6();
        VpnStatus {
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
        }
    }
}

pub fn connect(server: &VpnServer) -> Result<VpnStatus, String> {
    if is_vpn_running() {
        disconnect()?;
    }

    let config = generate_config(server);
    fs::write(OVPN_CONFIG_PATH, &config)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    fs::write(OVPN_AUTH_PATH, "vpn\nvpn\n")
        .map_err(|e| format!("Failed to write auth file: {}", e))?;

    platform::disable_ipv6()?;

    let output = Command::new("sudo")
        .args([
            "-n",
            "openvpn",
            "--config",
            OVPN_CONFIG_PATH,
            "--auth-user-pass",
            OVPN_AUTH_PATH,
            "--daemon",
            "--log",
            OVPN_LOG_PATH,
        ])
        .output()
        .map_err(|e| format!("Failed to start OpenVPN: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = format!("{} {}", stdout, stderr);
        if combined.contains("not allowed") || combined.contains("not permitted") {
            return Err("Permission denied. Please run Setup Permissions first.".to_string());
        }
        return Err(format!("OpenVPN failed to start: {}", combined.trim()));
    }

    let start = Instant::now();
    let timeout = Duration::from_secs(15);

    loop {
        if start.elapsed() > timeout {
            return Err("Connection timeout".to_string());
        }
        if is_vpn_running() {
            let sf = Some(server.filename.clone());
            let status = get_status(&Some(start), &sf);
            return Ok(status);
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn disconnect() -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["-n", "pkill", "-x", "openvpn"])
        .output()
        .map_err(|e| format!("Failed to kill OpenVPN: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("no process found") {
            log::warn!("pkill output: {}", stderr);
        }
    }

    platform::enable_ipv6()?;

    std::thread::sleep(Duration::from_secs(2));

    Ok(())
}

fn generate_config(server: &VpnServer) -> String {
    format!(
        r#"client
dev tun
proto {proto}
remote {ip} {port}
resolv-retry infinite
nobind
persist-key
persist-tun
cipher AES-128-CBC
data-ciphers AES-128-CBC
auth SHA1
verb 3

<ca>
{ca}
</ca>
"#,
        proto = server.proto,
        ip = server.ip,
        port = server.port,
        ca = CA_CERT
    )
}

pub fn get_vpn_ip() -> Option<String> {
    fetch_url_with_interface("https://ifconfig.me", "tun0")
}

pub fn get_real_ip() -> Option<String> {
    let output = Command::new("curl")
        .args(["-s", "--connect-timeout", "3", "--max-time", "5", "-4", "https://api.ipify.org"])
        .output()
        .ok()?;
    if output.status.success() {
        let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !ip.is_empty() {
            return Some(ip);
        }
    }
    None
}

pub fn get_real_ipv6() -> Option<String> {
    let output = Command::new("curl")
        .args(["-s", "--connect-timeout", "3", "--max-time", "5", "-6", "https://api64.ipify.org"])
        .output()
        .ok()?;
    if output.status.success() {
        let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !ip.is_empty() {
            return Some(ip);
        }
    }
    None
}

fn fetch_url_with_interface(url: &str, interface: &str) -> Option<String> {
    let output = Command::new("curl")
        .args([
            "-s",
            "--connect-timeout",
            "3",
            "--max-time",
            "5",
            "--interface",
            interface,
            url,
        ])
        .output()
        .ok()?;
    if output.status.success() {
        let body = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !body.is_empty() {
            return Some(body);
        }
    }
    None
}

pub fn get_data_usage(interface: &str, direction: &str) -> Option<String> {
    let content = fs::read_to_string("/proc/net/dev").ok()?;
    for line in content.lines() {
        if line.contains(interface) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                let bytes = match direction {
                    "rx" => parts[1].parse::<u64>().ok()?,
                    "tx" => parts[9].parse::<u64>().ok()?,
                    _ => return None,
                };
                let mb = bytes as f64 / 1_048_576.0;
                return Some(format!("{:.2} MB", mb));
            }
        }
    }
    None
}
