use crate::i18n::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut dashboard = HashMap::new();
    dashboard.insert("title".into(), "VPN Hizb".into());
    dashboard.insert("connection_overview".into(), "Connection Overview".into());
    dashboard.insert("status".into(), "Status".into());
    dashboard.insert("connected".into(), "Connected".into());
    dashboard.insert("disconnected".into(), "Disconnected".into());
    dashboard.insert("real_ip".into(), "Real IP".into());
    dashboard.insert("vpn_ip".into(), "VPN IP".into());
    dashboard.insert("country".into(), "Country".into());
    dashboard.insert("uptime".into(), "Uptime".into());
    dashboard.insert("proto".into(), "Protocol".into());
    dashboard.insert("interface".into(), "Interface".into());
    dashboard.insert("server".into(), "Server".into());
    dashboard.insert("data_in".into(), "Data In".into());
    dashboard.insert("data_out".into(), "Data Out".into());
    dashboard.insert("na".into(), "N/A".into());
    dashboard.insert("quick_start".into(), "Quick Start".into());
    dashboard.insert(
        "step1".into(),
        "Click Refresh in the server panel to fetch available servers".into(),
    );
    dashboard.insert(
        "step2".into(),
        "Select a server and click Connect".into(),
    );
    dashboard.insert(
        "step3".into(),
        "Wait for the connection to establish".into(),
    );
    dashboard.insert(
        "tip".into(),
        "Servers with low ping and high speed provide the best experience.".into(),
    );
    dashboard.insert("ip_leak_detected".into(), "IP Leak Detected".into());
    dashboard.insert(
        "ip_leak_desc".into(),
        "Real IP exposed on public network. Traffic may not be fully routed through VPN.".into(),
    );
    dashboard.insert(
        "secure".into(),
        "Secure -- No IP leak detected".into(),
    );
    dashboard.insert(
        "secure_desc".into(),
        "All traffic routed through VPN tunnel.".into(),
    );
    dashboard.insert("disconnect".into(), "Disconnect".into());
    dashboard.insert("disconnecting".into(), "Disconnecting...".into());
    dashboard.insert("confirm_connection".into(), "Confirm Connection".into());
    dashboard.insert(
        "connect_to".into(),
        "Connect to :server?".into(),
    );
    dashboard.insert("yes_connect".into(), "Yes, Connect".into());
    dashboard.insert("connecting".into(), "Connecting...".into());
    dashboard.insert("cancel".into(), "Cancel".into());
    dashboard.insert("retry".into(), "Retry".into());
    dashboard.insert("servers".into(), "Servers (:count)".into());
    dashboard.insert("refresh".into(), "Refresh".into());
    dashboard.insert("refreshing".into(), "Refreshing...".into());
    dashboard.insert(
        "refresh_in_progress".into(),
        "Refresh in progress...".into(),
    );
    dashboard.insert(
        "no_servers".into(),
        "No servers found. Click Refresh to fetch from VPN Gate API.".into(),
    );
    dashboard.insert("ping".into(), "Ping: :value ms".into());
    dashboard.insert("total".into(), "Total: :value".into());
    dashboard.insert("selected".into(), "Selected".into());
    dashboard.insert("connect".into(), "Connect".into());
    dashboard.insert("credit".into(), "About".into());
    dashboard.insert("settings".into(), "Settings".into());
    dashboard.insert("about_title".into(), "VPN Hizb".into());
    dashboard.insert("about_version".into(), "Version".into());
    dashboard.insert("about_author".into(), "Author".into());
    dashboard.insert("about_email".into(), "Email".into());
    dashboard.insert("about_license".into(), "License".into());
    dashboard.insert("about_dedication".into(), "Dedication".into());
    dashboard.insert(
        "about_dedication_text".into(),
        "This application was made with love for Siti Sri Fitriani.".into(),
    );
    dashboard.insert(
        "about_description".into(),
        "Secure internet access through a global network of public VPN servers.".into(),
    );
    dashboard.insert(
        "failed_refresh".into(),
        "Failed to start refresh".into(),
    );
    dashboard.insert(
        "failed_connect".into(),
        "Failed to connect".into(),
    );
    dashboard.insert(
        "failed_disconnect".into(),
        "Failed to disconnect".into(),
    );
    dashboard.insert(
        "sudoers_title".into(),
        "Initial Setup Required".into(),
    );
    dashboard.insert(
        "sudoers_desc".into(),
        "VPN Hizb needs system permissions to manage OpenVPN. Click below to set up — a system dialog will ask for your password.".into(),
    );
    dashboard.insert(
        "sudoers_setup".into(),
        "Setup Permissions".into(),
    );
    dashboard.insert(
        "sudoers_waiting".into(),
        "Waiting for password...".into(),
    );
    dashboard.insert(
        "sudoers_skip".into(),
        "Skip for now".into(),
    );
    dashboard.insert(
        "sudoers_error".into(),
        "Setup failed. You can run this command manually in a terminal:".into(),
    );
    dashboard.insert(
        "sudoers_retry".into(),
        "Try Again".into(),
    );
    dashboard.insert("close".into(), "Close".into());

    let mut server = HashMap::new();
    server.insert(
        "refresh_already_running".into(),
        "Refresh already in progress".into(),
    );
    server.insert("refresh_started".into(), "Refresh started".into());
    server.insert(
        "connected".into(),
        "Connected".into(),
    );
    server.insert(
        "disconnected".into(),
        "Disconnected".into(),
    );
    server.insert(
        "connection_timeout".into(),
        "Connection timeout".into(),
    );
    server.insert(
        "server_not_found".into(),
        "Server not found".into(),
    );

    Translations {
        dashboard,
        server,
    }
}
