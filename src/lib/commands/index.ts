import { invoke } from '@tauri-apps/api/core';

export type VpnStatus = {
    connected: boolean;
    ip: string | null;
    real_ip: string | null;
    real_ipv6: string | null;
    country: string | null;
    proto: string | null;
    uptime: string | null;
    data_in: string | null;
    data_out: string | null;
    server: string | null;
    interface: string | null;
    leak: boolean;
    ping: number | null;
    speed: number | null;
};

export type VpnServer = {
    name: string;
    filename: string;
    country: string;
    country_code: string | null;
    hostname: string;
    ip: string | null;
    ping: number | null;
    speed: number | null;
    proto: string | null;
    operator: string | null;
};

export type Translations = {
    dashboard: Record<string, string>;
    server: Record<string, string>;
};

export type LocaleResponse = {
    locale: string;
    translations: Translations;
};

export type RefreshStatus = {
    running: boolean;
    last_success: number | null;
};

export async function getStatus(): Promise<VpnStatus> {
    return invoke('get_status');
}

export async function getServers(): Promise<VpnServer[]> {
    return invoke('get_servers');
}

export async function connectToServer(server: string): Promise<VpnStatus> {
    return invoke('connect', { server });
}

export async function disconnectVpn(): Promise<void> {
    return invoke('disconnect');
}

export async function refreshServers(): Promise<RefreshStatus> {
    return invoke('refresh_servers');
}

export async function getRefreshStatus(): Promise<RefreshStatus> {
    return invoke('get_refresh_status');
}

export async function setLocale(locale: string): Promise<LocaleResponse> {
    return invoke('set_locale', { locale });
}

export async function getTranslations(locale: string): Promise<Translations> {
    return invoke('get_translations', { locale });
}

export type SudoersStatus = {
    configured: boolean;
};

export async function checkSudoers(): Promise<SudoersStatus> {
    return invoke('check_sudoers');
}

export async function setupSudoers(): Promise<SudoersStatus> {
    return invoke('setup_sudoers');
}
