use crate::i18n::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut dashboard = HashMap::new();
    dashboard.insert("title".into(), "VPN Hizb".into());
    dashboard.insert("connection_overview".into(), "Ringkasan Koneksi".into());
    dashboard.insert("status".into(), "Status".into());
    dashboard.insert("connected".into(), "Terhubung".into());
    dashboard.insert("disconnected".into(), "Terputus".into());
    dashboard.insert("real_ip".into(), "IP Asli".into());
    dashboard.insert("vpn_ip".into(), "IP VPN".into());
    dashboard.insert("country".into(), "Negara".into());
    dashboard.insert("uptime".into(), "Waktu Aktif".into());
    dashboard.insert("proto".into(), "Protokol".into());
    dashboard.insert("interface".into(), "Interface".into());
    dashboard.insert("server".into(), "Server".into());
    dashboard.insert("data_in".into(), "Data Masuk".into());
    dashboard.insert("data_out".into(), "Data Keluar".into());
    dashboard.insert("na".into(), "N/A".into());
    dashboard.insert("quick_start".into(), "Mulai Cepat".into());
    dashboard.insert(
        "step1".into(),
        "Klik Muat Ulang di panel server untuk mengambil server yang tersedia".into(),
    );
    dashboard.insert(
        "step2".into(),
        "Pilih server dan klik Hubungkan".into(),
    );
    dashboard.insert(
        "step3".into(),
        "Tunggu koneksi terbentuk".into(),
    );
    dashboard.insert(
        "tip".into(),
        "Server dengan ping rendah dan kecepatan tinggi memberikan pengalaman terbaik.".into(),
    );
    dashboard.insert("ip_leak_detected".into(), "Kebocoran IP Terdeteksi".into());
    dashboard.insert(
        "ip_leak_desc".into(),
        "IP Asli terekspos di jaringan publik. Lalu lintas mungkin tidak sepenuhnya dirutekan melalui VPN.".into(),
    );
    dashboard.insert(
        "secure".into(),
        "Aman -- Tidak ada kebocoran IP".into(),
    );
    dashboard.insert(
        "secure_desc".into(),
        "Semua lalu lintas dirutekan melalui tunnel VPN.".into(),
    );
    dashboard.insert("disconnect".into(), "Putuskan".into());
    dashboard.insert("disconnecting".into(), "Memutuskan...".into());
    dashboard.insert("confirm_connection".into(), "Konfirmasi Koneksi".into());
    dashboard.insert(
        "connect_to".into(),
        "Hubungkan ke :server?".into(),
    );
    dashboard.insert("yes_connect".into(), "Ya, Hubungkan".into());
    dashboard.insert("connecting".into(), "Menghubungkan...".into());
    dashboard.insert("cancel".into(), "Batal".into());
    dashboard.insert("retry".into(), "Coba Lagi".into());
    dashboard.insert("servers".into(), "Server (:count)".into());
    dashboard.insert("refresh".into(), "Muat Ulang".into());
    dashboard.insert("refreshing".into(), "Memuat ulang...".into());
    dashboard.insert(
        "refresh_in_progress".into(),
        "Pembaruan sedang berlangsung...".into(),
    );
    dashboard.insert(
        "no_servers".into(),
        "Tidak ada server ditemukan. Klik Muat Ulang untuk mengambil dari VPN Gate API.".into(),
    );
    dashboard.insert("ping".into(), "Ping: :value ms".into());
    dashboard.insert("total".into(), "Total: :value".into());
    dashboard.insert("selected".into(), "Dipilih".into());
    dashboard.insert("connect".into(), "Hubungkan".into());
    dashboard.insert("credit".into(), "Tentang".into());
    dashboard.insert("settings".into(), "Pengaturan".into());
    dashboard.insert("about_title".into(), "VPN Hizb".into());
    dashboard.insert("about_version".into(), "Versi".into());
    dashboard.insert("about_author".into(), "Penulis".into());
    dashboard.insert("about_email".into(), "Email".into());
    dashboard.insert("about_license".into(), "Lisensi".into());
    dashboard.insert("about_dedication".into(), "Dedikasi".into());
    dashboard.insert(
        "about_dedication_text".into(),
        "Aplikasi ini dibuat dengan penuh cinta untuk Siti Sri Fitriani.".into(),
    );
    dashboard.insert(
        "about_description".into(),
        "Akses internet aman melalui jaringan server VPN publik global.".into(),
    );
    dashboard.insert(
        "failed_refresh".into(),
        "Gagal memuat ulang".into(),
    );
    dashboard.insert(
        "failed_connect".into(),
        "Gagal menghubungkan".into(),
    );
    dashboard.insert(
        "failed_disconnect".into(),
        "Gagal memutuskan".into(),
    );
    dashboard.insert(
        "sudoers_title".into(),
        "Pengaturan Awal Diperlukan".into(),
    );
    dashboard.insert(
        "sudoers_desc".into(),
        "VPN Hizb memerlukan izin sistem untuk mengelola OpenVPN. Klik di bawah untuk mengatur — dialog sistem akan meminta kata sandi Anda.".into(),
    );
    dashboard.insert(
        "sudoers_setup".into(),
        "Atur Izin".into(),
    );
    dashboard.insert(
        "sudoers_waiting".into(),
        "Menunggu kata sandi...".into(),
    );
    dashboard.insert(
        "sudoers_skip".into(),
        "Lewati untuk sekarang".into(),
    );
    dashboard.insert(
        "sudoers_error".into(),
        "Pengaturan gagal. Anda dapat menjalankan perintah ini secara manual di terminal:".into(),
    );
    dashboard.insert(
        "sudoers_retry".into(),
        "Coba Lagi".into(),
    );
    dashboard.insert("close".into(), "Tutup".into());

    let mut server = HashMap::new();
    server.insert(
        "refresh_already_running".into(),
        "Pembaruan sedang berlangsung".into(),
    );
    server.insert("refresh_started".into(), "Pembaruan dimulai".into());
    server.insert(
        "connected".into(),
        "Terhubung".into(),
    );
    server.insert(
        "disconnected".into(),
        "Terputus".into(),
    );
    server.insert(
        "connection_timeout".into(),
        "Koneksi habis waktu".into(),
    );
    server.insert(
        "server_not_found".into(),
        "Server tidak ditemukan".into(),
    );

    Translations {
        dashboard,
        server,
    }
}
