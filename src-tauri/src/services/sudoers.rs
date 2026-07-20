use std::fs;
use std::process::Command;

fn get_current_user() -> String {
    Command::new("whoami")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "root".to_string())
}

fn generate_sudoers_content() -> String {
    let user = get_current_user();
    format!(
        "# VPN Hizb sudoers configuration\n\
         {user} ALL=(root) NOPASSWD: /usr/sbin/openvpn *\n\
         {user} ALL=(root) NOPASSWD: /usr/sbin/sysctl *\n\
         {user} ALL=(root) NOPASSWD: /usr/bin/pkill *\n",
        user = user
    )
}

pub fn is_sudoers_configured() -> bool {
    let output = Command::new("sudo")
        .args(["-n", "openvpn", "--version"])
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

pub fn setup_sudoers() -> Result<(), String> {
    let content = generate_sudoers_content();
    let tmp_path = "/tmp/vpn-hizb-sudoers";
    fs::write(tmp_path, &content)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    let script = format!(
        "cp {} /etc/sudoers.d/vpn-hizb && chmod 440 /etc/sudoers.d/vpn-hizb",
        tmp_path
    );

    let output = Command::new("pkexec")
        .args(["sh", "-c", &script])
        .output()
        .map_err(|e| format!("Failed to run pkexec: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = format!("{} {}", stdout, stderr);
        let _ = fs::remove_file(tmp_path);
        if combined.contains("not found") || combined.contains("No such file") || combined.contains("not installed") {
            return Err(format!(
                "pkexec not available. Please run this in a terminal:\n\
                 sudo tee /etc/sudoers.d/vpn-hizb <<'EOF'\n\
                 {}\
                 EOF\n\
                 sudo chmod 440 /etc/sudoers.d/vpn-hizb",
                content
            ));
        }
        return Err(format!("Failed to setup sudoers: {}", combined.trim()));
    }

    let _ = fs::remove_file(tmp_path);

    Ok(())
}
