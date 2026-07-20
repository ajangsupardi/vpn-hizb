use std::process::Command;

pub fn disable_ipv6() -> Result<(), String> {
    run_sysctl("net.ipv6.conf.all.disable_ipv6", "1")?;
    run_sysctl("net.ipv6.conf.default.disable_ipv6", "1")?;
    Ok(())
}

pub fn enable_ipv6() -> Result<(), String> {
    run_sysctl("net.ipv6.conf.all.disable_ipv6", "0")?;
    run_sysctl("net.ipv6.conf.default.disable_ipv6", "0")?;
    Ok(())
}

fn run_sysctl(key: &str, value: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["-n", "sysctl", "-w", &format!("{}={}", key, value)])
        .output()
        .map_err(|e| format!("Failed to run sysctl: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("sysctl failed for {}: {}", key, stderr);
    }
    Ok(())
}
