<script lang="ts">
    import AppHead from '../components/AppHead.svelte';
    import { createI18n } from '../lib/i18n';
    import type { VpnStatus, VpnServer, Translations } from '../lib/commands/index';
    import {
        getStatus,
        getServers,
        connectToServer as apiConnect,
        disconnectVpn as apiDisconnect,
        refreshServers as apiRefresh,
        getRefreshStatus,
        setLocale as apiSetLocale,
        checkSudoers,
        setupSudoers as apiSetupSudoers,
    } from '../lib/commands/index';

    let currentLocale = $state('en');
    let currentTranslations = $state<Translations>({ dashboard: {}, server: {} });
    let switchingLocale = $state(false);

    let i18n = $derived(createI18n(currentTranslations, currentLocale));
    let t = $derived(i18n.t);

    let vpnStatus = $state<VpnStatus>({
        connected: false, ip: null, real_ip: null, real_ipv6: null,
        country: null, proto: null, uptime: null, data_in: null,
        data_out: null, server: null, interface: null, leak: false,
        ping: null, speed: null,
    });
    let vpnServers = $state<VpnServer[]>([]);
    let loading = $state(true);

    let connecting = $state(false);
    let disconnecting = $state(false);
    let refreshing = $state(false);
    let refreshInBackground = $state(false);
    let error = $state('');
    let confirmServer = $state<string | null>(null);
    let confirmName = $state<string | null>(null);
    let refreshPolling = $state(false);
    let showCreditModal = $state(false);
    let savedRealIp = $state<string | null>(null);

    let sudoersConfigured = $state(true);
    let showSudoersModal = $state(false);
    let settingUpSudoers = $state(false);
    let sudoersError = $state('');

    async function loadData() {
        try {
            const [status, servers, translations, sudoers] = await Promise.all([
                getStatus(),
                getServers(),
                getTranslations('en'),
                checkSudoers(),
            ]);
            vpnStatus = status;
            vpnServers = servers;
            currentTranslations = translations;
            savedRealIp = status.real_ip;
            sudoersConfigured = sudoers.configured;
            if (!sudoers.configured) {
                showSudoersModal = true;
            }
        } catch (e) {
            console.error('Failed to load data:', e);
        } finally {
            loading = false;
        }
    }

    async function getTranslations(locale: string): Promise<Translations> {
        const { invoke } = await import('@tauri-apps/api/core');
        return invoke('get_translations', { locale });
    }

    async function checkRefreshStatus() {
        try {
            const data = await getRefreshStatus();
            if (data.running) {
                refreshInBackground = true;
                refreshing = false;
                if (!refreshPolling) {
                    refreshPolling = true;
                    startRefreshPolling();
                }
            } else {
                if (refreshInBackground) {
                    await refreshServerList();
                }
                refreshInBackground = false;
                refreshing = false;
                refreshPolling = false;
            }
        } catch {
            // ignore
        }
    }

    async function refreshStatus() {
        try {
            vpnStatus = await getStatus();
        } catch {
            // ignore
        }
    }

    async function refreshServers() {
        error = '';
        try {
            const data = await apiRefresh();
            if (data.running) {
                refreshing = true;
                refreshInBackground = false;
                refreshPolling = true;
                startRefreshPolling();
            } else {
                refreshing = false;
                refreshInBackground = false;
                await refreshServerList();
            }
        } catch {
            error = t('dashboard.failed_refresh');
        }
    }

    function startRefreshPolling() {
        const interval = setInterval(async () => {
            await checkRefreshStatus();
            if (!refreshPolling) {
                clearInterval(interval);
            }
        }, 3000);
    }

    async function refreshServerList() {
        try {
            vpnServers = await getServers();
        } catch {
            // ignore
        }
    }

    async function connectToServer(filename: string, name: string) {
        error = '';
        confirmServer = filename;
        confirmName = name;
    }

    async function confirmConnect() {
        if (!confirmServer) return;

        const serverFile = confirmServer;
        connecting = true;
        error = '';

        try {
            const status = await getStatus();
            if (status.real_ip) {
                savedRealIp = status.real_ip;
            }

            vpnStatus = await apiConnect(serverFile);
            confirmServer = null;
        } catch (e: any) {
            error = typeof e === 'string' ? e : (e?.message || t('dashboard.failed_connect'));
        } finally {
            connecting = false;
        }
    }

    async function disconnectVpn() {
        disconnecting = true;
        error = '';
        try {
            await apiDisconnect();
            vpnStatus = await getStatus();
        } catch (e: any) {
            error = typeof e === 'string' ? e : (e?.message || t('dashboard.failed_disconnect'));
        } finally {
            disconnecting = false;
        }
    }

    async function cancelConfirm() {
        if (connecting) {
            await disconnectVpn();
        }
        confirmServer = null;
    }

    async function switchLocale() {
        if (switchingLocale) return;
        switchingLocale = true;
        try {
            const newLocale = currentLocale === 'en' ? 'id' : 'en';
            const result = await apiSetLocale(newLocale);
            currentLocale = result.locale;
            currentTranslations = result.translations;
        } finally {
            switchingLocale = false;
        }
    }

    async function handleSetupSudoers() {
        settingUpSudoers = true;
        sudoersError = '';
        try {
            await apiSetupSudoers();
            sudoersConfigured = true;
            showSudoersModal = false;
        } catch (e) {
            sudoersError = String(e);
        } finally {
            settingUpSudoers = false;
        }
    }

    $effect(() => {
        loadData();
        checkRefreshStatus();
        const interval = setInterval(refreshStatus, 10000);
        return () => clearInterval(interval);
    });
</script>

<AppHead title={t('dashboard.title')} />

{#if loading}
    <div class="flex min-h-screen items-center justify-center bg-surface-alt">
        <div class="flex flex-col items-center gap-5">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" class="h-16 w-16">
                <path fill="#48b9c7" fill-rule="evenodd" d="M 7.438,5.702 C 9.816,5.17 14.678,4.5 24,4.5 c 9.322,0 14.184,0.67 16.562,1.202 0.605,0.135 1.13,0.653 1.239,1.661 0.297,2.759 0.699,7.44 0.699,12.532 q 0,0.729 -0.01,1.434 C 42.377,29.035 38.473,36.16 31.961,40.152 30.364,41.132 28.79,41.988 27.365,42.594 25.902,43.217 24.768,43.5 24,43.5 c -0.768,0 -1.902,-0.283 -3.365,-0.906 -1.425,-0.606 -3,-1.463 -4.596,-2.442 C 9.527,36.16 5.623,29.035 5.51,21.33 Q 5.5,20.625 5.499,19.896 c 0,-5.091 0.402,-9.773 0.7,-12.532 C 6.307,6.356 6.832,5.838 7.437,5.703 M 24,0.5 C 14.52,0.5 9.336,1.179 6.565,1.798 3.921,2.39 2.47,4.63 2.222,6.935 1.916,9.776 1.5,14.611 1.5,19.895 q 0,0.76 0.011,1.493 c 0.132,9.013 4.704,17.434 12.437,22.174 1.691,1.037 3.45,2.002 5.121,2.713 1.634,0.695 3.359,1.225 4.931,1.225 1.572,0 3.297,-0.53 4.93,-1.225 1.672,-0.711 3.431,-1.676 5.122,-2.713 7.733,-4.74 12.305,-13.16 12.437,-22.174 q 0.01,-0.735 0.011,-1.492 C 46.5,14.611 46.084,9.776 45.778,6.935 45.53,4.63 44.079,2.389 41.435,1.798 38.663,1.178 33.48,0.5 24,0.5 m -8.693,10.804 c 1.7,-0.268 4.415,-0.51 8.693,-0.51 4.278,0 6.994,0.242 8.693,0.51 1.924,0.303 3.07,1.88 3.24,3.72 0.147,1.606 0.294,3.802 0.294,6.155 0,5.452 -2.766,10.596 -7.518,13.321 -0.84,0.482 -1.684,0.91 -2.477,1.218 -0.786,0.306 -1.555,0.509 -2.232,0.509 -0.677,0 -1.446,-0.203 -2.232,-0.509 -0.792,-0.308 -1.637,-0.736 -2.477,-1.218 -4.752,-2.725 -7.518,-7.87 -7.518,-13.321 0,-2.353 0.148,-4.55 0.295,-6.156 0.17,-1.838 1.316,-3.416 3.24,-3.719" clip-rule="evenodd" />
            </svg>
            <h1 class="font-serif text-2xl font-bold text-primary">VPN Hizb</h1>
            <div class="flex items-center gap-2 text-text-secondary">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="animate-spin"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>
                <span class="font-sans text-sm">Loading...</span>
            </div>
        </div>
    </div>
{:else}
<div class="min-h-screen bg-surface-alt">
    <header class="border-b border-border bg-surface shadow-raised">
        <div class="mx-auto flex max-w-[1280px] items-center justify-between px-4 py-3 sm:px-6 lg:px-10">
            <div class="flex items-center gap-3">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" class="h-6 w-6">
                    <path fill="#48b9c7" fill-rule="evenodd" d="M 7.438,5.702 C 9.816,5.17 14.678,4.5 24,4.5 c 9.322,0 14.184,0.67 16.562,1.202 0.605,0.135 1.13,0.653 1.239,1.661 0.297,2.759 0.699,7.44 0.699,12.532 q 0,0.729 -0.01,1.434 C 42.377,29.035 38.473,36.16 31.961,40.152 30.364,41.132 28.79,41.988 27.365,42.594 25.902,43.217 24.768,43.5 24,43.5 c -0.768,0 -1.902,-0.283 -3.365,-0.906 -1.425,-0.606 -3,-1.463 -4.596,-2.442 C 9.527,36.16 5.623,29.035 5.51,21.33 Q 5.5,20.625 5.499,19.896 c 0,-5.091 0.402,-9.773 0.7,-12.532 C 6.307,6.356 6.832,5.838 7.437,5.703 M 24,0.5 C 14.52,0.5 9.336,1.179 6.565,1.798 3.921,2.39 2.47,4.63 2.222,6.935 1.916,9.776 1.5,14.611 1.5,19.895 q 0,0.76 0.011,1.493 c 0.132,9.013 4.704,17.434 12.437,22.174 1.691,1.037 3.45,2.002 5.121,2.713 1.634,0.695 3.359,1.225 4.931,1.225 1.572,0 3.297,-0.53 4.93,-1.225 1.672,-0.711 3.431,-1.676 5.122,-2.713 7.733,-4.74 12.305,-13.16 12.437,-22.174 q 0.01,-0.735 0.011,-1.492 C 46.5,14.611 46.084,9.776 45.778,6.935 45.53,4.63 44.079,2.389 41.435,1.798 38.663,1.178 33.48,0.5 24,0.5 m -8.693,10.804 c 1.7,-0.268 4.415,-0.51 8.693,-0.51 4.278,0 6.994,0.242 8.693,0.51 1.924,0.303 3.07,1.88 3.24,3.72 0.147,1.606 0.294,3.802 0.294,6.155 0,5.452 -2.766,10.596 -7.518,13.321 -0.84,0.482 -1.684,0.91 -2.477,1.218 -0.786,0.306 -1.555,0.509 -2.232,0.509 -0.677,0 -1.446,-0.203 -2.232,-0.509 -0.792,-0.308 -1.637,-0.736 -2.477,-1.218 -4.752,-2.725 -7.518,-7.87 -7.518,-13.321 0,-2.353 0.148,-4.55 0.295,-6.156 0.17,-1.838 1.316,-3.416 3.24,-3.719" clip-rule="evenodd" />
                </svg>
                <h1 class="font-serif text-xl font-bold text-primary">{t('dashboard.title')}</h1>
            </div>
            <div class="flex items-center gap-1">
                <button
                    onclick={() => showSudoersModal = true}
                    class="inline-flex items-center gap-1.5 rounded bg-transparent px-3 py-2 font-serif text-sm font-normal text-primary-soft transition-colors hover:bg-surface-alt hover:text-teal"
                    title={t('dashboard.settings')}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
                    <span class="text-xs font-bold">{t('dashboard.settings')}</span>
                </button>
                <button
                    onclick={switchLocale}
                    disabled={switchingLocale}
                    class="inline-flex items-center gap-1.5 rounded bg-transparent px-3 py-2 font-serif text-sm font-normal text-primary-soft transition-colors hover:bg-surface-alt hover:text-teal disabled:cursor-not-allowed disabled:opacity-50"
                    title={currentLocale === 'en' ? 'Switch to Indonesian' : 'Switch to English'}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/></svg>
                    <span class="text-xs font-bold">{switchingLocale ? '...' : (currentLocale === 'en' ? 'ID' : 'EN')}</span>
                </button>
                <button
                    onclick={() => showCreditModal = true}
                    class="inline-flex items-center gap-2 rounded bg-transparent px-3 py-2 font-serif text-sm font-normal text-primary-soft transition-colors hover:bg-surface-alt hover:text-teal"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
                    {t('dashboard.credit')}
                </button>
            </div>
        </div>
    </header>

    <main class="mx-auto max-w-[1280px] px-4 py-8 sm:px-6 lg:px-10">
        <div class="grid gap-6 lg:grid-cols-3">
            <div class="lg:col-span-2">
                <div class="rounded-xl bg-surface p-6 shadow-elevated transition-shadow hover:shadow-modal sm:p-8">
                    <h2 class="mb-6 font-serif text-lg font-bold text-primary">
                        {t('dashboard.connection_overview')}
                    </h2>

                    <div class="grid gap-4 sm:grid-cols-2">
                        <div class="rounded-lg border border-border bg-surface-alt p-4">
                            <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.status')}</div>
                            <div class="mt-2 flex items-center gap-2">
                                <span class="h-2.5 w-2.5 rounded-full {vpnStatus.connected ? 'bg-teal' : 'bg-border-strong'}"></span>
                                <span class="font-serif font-bold {vpnStatus.connected ? 'text-teal' : 'text-text-secondary'}">
                                    {vpnStatus.connected ? t('dashboard.connected') : t('dashboard.disconnected')}
                                </span>
                            </div>
                        </div>

                        <div class="rounded-lg border border-border bg-surface-alt p-4">
                            <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.real_ip')}</div>
                            <div class="mt-2 font-mono text-sm text-primary">
                                {savedRealIp ?? vpnStatus.real_ip ?? t('dashboard.na')}
                            </div>
                        </div>
                    </div>

                    {#if vpnStatus.connected}
                        <div class="mt-4 grid gap-4 sm:grid-cols-2">
                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.vpn_ip')}</div>
                                <div class="mt-2 font-mono text-sm text-primary">
                                    {vpnStatus.ip ?? t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.country')}</div>
                                <div class="mt-2 font-sans text-sm text-primary">
                                    {vpnStatus.country && vpnStatus.country !== 'N/A' ? vpnStatus.country : t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.uptime')}</div>
                                <div class="mt-2 font-mono text-sm text-primary">
                                    {vpnStatus.uptime ?? t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.proto')}</div>
                                <div class="mt-2 font-sans text-sm text-primary">
                                    {vpnStatus.proto?.toUpperCase() ?? t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.interface')}</div>
                                <div class="mt-2 font-mono text-sm text-primary">
                                    {vpnStatus.interface ?? t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.server')}</div>
                                <div class="mt-2 font-mono text-sm text-primary truncate">
                                    {vpnStatus.server ?? t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.data_in')}</div>
                                <div class="mt-2 font-mono text-sm text-primary">
                                    {vpnStatus.data_in ?? t('dashboard.na')}
                                </div>
                            </div>

                            <div class="rounded-lg border border-border bg-surface-alt p-4">
                                <div class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.data_out')}</div>
                                <div class="mt-2 font-mono text-sm text-primary">
                                    {vpnStatus.data_out ?? t('dashboard.na')}
                                </div>
                            </div>
                        </div>
                    {:else}
                        <div class="mt-4 rounded-lg border border-border bg-surface-alt p-6">
                            <div class="mb-4 flex items-center gap-3">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-teal"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>
                                <p class="font-serif font-bold text-primary">{t('dashboard.quick_start')}</p>
                            </div>
                            <ol class="space-y-3 font-sans text-sm text-primary-soft">
                                <li class="flex items-start gap-3">
                                    <span class="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-teal font-serif text-xs font-bold text-white">1</span>
                                    <span>{t('dashboard.step1')}</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-teal font-serif text-xs font-bold text-white">2</span>
                                    <span>{t('dashboard.step2')}</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-teal font-serif text-xs font-bold text-white">3</span>
                                    <span>{t('dashboard.step3')}</span>
                                </li>
                            </ol>
                            <div class="mt-4 flex items-start gap-2 rounded border border-teal bg-[#E8F4F8] p-3">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mt-0.5 shrink-0 text-teal"><path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6"/><path d="M10 22h4"/></svg>
                                <p class="font-sans text-xs text-primary-soft">{t('dashboard.tip')}</p>
                            </div>
                        </div>
                    {/if}

                    {#if vpnStatus.connected}
                        {#if vpnStatus.leak}
                            <div class="mt-4 rounded-lg border border-error bg-[#FFEBEE] p-4">
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-error"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>
                                    <div>
                                        <p class="font-serif font-bold text-error">{t('dashboard.ip_leak_detected')}</p>
                                        <p class="font-sans text-xs text-primary-soft">{t('dashboard.ip_leak_desc')}</p>
                                    </div>
                                </div>
                            </div>
                        {:else}
                            <div class="mt-4 rounded-lg border border-teal bg-[#E8F4F8] p-4">
                                <div class="flex items-center gap-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-teal"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="m9 12 2 2 4-4"/></svg>
                                    <div>
                                        <p class="font-serif font-bold text-teal">{t('dashboard.secure')}</p>
                                        <p class="font-sans text-xs text-primary-soft">{t('dashboard.secure_desc')}</p>
                                    </div>
                                </div>
                            </div>
                        {/if}
                    {/if}

                    <div class="mt-6 flex gap-3">
                        {#if vpnStatus.connected}
                            <button
                                onclick={disconnectVpn}
                                disabled={disconnecting}
                                class="inline-flex items-center gap-2 rounded bg-error px-5 py-3 font-serif text-sm font-normal text-white transition-colors hover:bg-[#E02D2D] disabled:cursor-not-allowed disabled:bg-border-strong disabled:text-[#999999]"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18.36 6.64a9 9 0 1 1-12.73 0"/><line x1="12" x2="12" y1="2" y2="12"/></svg>
                                {disconnecting ? t('dashboard.disconnecting') : t('dashboard.disconnect')}
                            </button>
                        {/if}
                    </div>

                    {#if confirmServer}
                        <div class="mt-4 rounded border {error ? 'border-error bg-[#FFEBEE]' : 'border-gold bg-[#FFF3E0]'} p-4">
                            {#if error}
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-error"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>
                                        <span class="font-sans text-sm font-medium text-error">{error}</span>
                                    </div>
                                    <div class="flex gap-2">
                                        <button
                                            onclick={confirmConnect}
                                            class="rounded bg-gold px-3 py-1.5 font-serif text-xs font-normal text-black transition-colors hover:bg-gold-alt"
                                        >
                                            {t('dashboard.retry')}
                                        </button>
                                        <button
                                            onclick={() => { error = ''; confirmServer = null; }}
                                            class="rounded px-2 py-1 text-error hover:bg-[#FFEBEE] hover:text-[#E02D2D]"
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                                        </button>
                                    </div>
                                </div>
                            {:else}
                                <div class="flex items-center justify-between">
                                    <div>
                                        <p class="font-serif font-bold text-primary">{t('dashboard.confirm_connection')}</p>
                                        <p class="font-sans text-sm text-primary-soft">{t('dashboard.connect_to', { server: confirmName ?? confirmServer ?? '' })}</p>
                                    </div>
                                    <div class="flex gap-3">
                                        <button
                                            onclick={confirmConnect}
                                            disabled={connecting}
                                            class="inline-flex items-center gap-2 rounded bg-gold px-4 py-2 font-serif text-sm font-normal text-black transition-colors hover:bg-gold-alt disabled:cursor-not-allowed disabled:bg-border-strong disabled:text-[#999999]"
                                        >
                                            {#if connecting}
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="animate-spin"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
                                                {t('dashboard.connecting')}
                                            {:else}
                                                {t('dashboard.yes_connect')}
                                            {/if}
                                        </button>
                                        <button
                                            onclick={cancelConfirm}
                                            class="rounded border border-primary bg-transparent px-4 py-2 font-serif text-sm font-normal text-primary transition-colors hover:bg-surface-alt"
                                        >
                                            {t('dashboard.cancel')}
                                        </button>
                                    </div>
                                </div>
                            {/if}
                        </div>
                    {/if}
                </div>
            </div>

            <div>
                <div class="rounded-xl bg-surface p-6 shadow-elevated transition-shadow hover:shadow-modal sm:p-8">
                    <div class="mb-4 flex items-center justify-between">
                        <h2 class="font-serif text-lg font-bold text-primary">
                            {t('dashboard.servers', { count: vpnServers.length })}
                        </h2>
                        <button
                            onclick={refreshServers}
                            disabled={refreshing || refreshInBackground}
                            class="inline-flex items-center gap-1.5 rounded border border-primary bg-transparent px-3 py-1.5 font-serif text-xs font-normal text-primary transition-colors hover:bg-surface-alt disabled:cursor-not-allowed disabled:border-border-strong disabled:text-border-strong"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={refreshing || refreshInBackground ? 'animate-spin' : ''}><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>
                            {#if refreshing}
                                {t('dashboard.refreshing')}
                            {:else if refreshInBackground}
                                {t('dashboard.refresh_in_progress')}
                            {:else}
                                {t('dashboard.refresh')}
                            {/if}
                        </button>
                    </div>

                    <div class="max-h-[600px] space-y-2 overflow-y-auto">
                        {#each vpnServers as server (server.filename)}
                            <div class="rounded-lg border {confirmServer === server.filename ? 'border-gold bg-[#FFF3E0]' : 'border-border bg-surface-alt'} p-3 transition-all hover:border-teal hover:shadow-elevated">
                                <div class="flex items-center justify-between">
                                    <div class="min-w-0 flex-1">
                                        <div class="flex items-center gap-2">
                                            <span class="truncate font-sans text-sm font-medium text-primary">{server.country}</span>
                                            {#if server.country_code}
                                                <span class="text-xs text-text-secondary">({server.country_code})</span>
                                            {/if}
                                        </div>
                                        <div class="mt-1 font-mono text-xs text-text-secondary">
                                            {server.hostname}
                                        </div>
                                        <div class="mt-1 flex gap-3 font-sans text-xs text-text-secondary">
                                            <span>{t('dashboard.ping', { value: server.ping ?? 'N/A' })}</span>
                                            <span>{t('dashboard.total', { value: server.speed !== null && server.speed !== undefined ? (server.speed / 1073741824).toFixed(2) + ' GB' : 'N/A' })}</span>
                                        </div>
                                    </div>
                                    <button
                                        onclick={() => connectToServer(server.filename, server.name)}
                                        disabled={connecting || vpnStatus.connected}
                                        class="ml-3 shrink-0 rounded px-3 py-1.5 font-serif text-xs font-normal text-black transition-colors disabled:cursor-not-allowed disabled:bg-border-strong disabled:text-[#999999] {confirmServer === server.filename ? 'bg-teal text-white hover:bg-teal-light' : 'bg-gold hover:bg-gold-alt'}"
                                    >
                                        {#if confirmServer === server.filename}
                                            {t('dashboard.selected')}
                                        {:else}
                                            {t('dashboard.connect')}
                                        {/if}
                                    </button>
                                </div>
                            </div>
                        {/each}

                        {#if vpnServers.length === 0}
                            <div class="py-8 text-center font-sans text-sm text-text-secondary">
                                {t('dashboard.no_servers')}
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </main>
</div>
{/if}

{#if showCreditModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div class="mx-4 w-full max-w-lg rounded-xl bg-surface p-8 shadow-floating">
            <div class="mb-6 flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" class="h-8 w-8">
                        <path fill="#48b9c7" fill-rule="evenodd" d="M 7.438,5.702 C 9.816,5.17 14.678,4.5 24,4.5 c 9.322,0 14.184,0.67 16.562,1.202 0.605,0.135 1.13,0.653 1.239,1.661 0.297,2.759 0.699,7.44 0.699,12.532 q 0,0.729 -0.01,1.434 C 42.377,29.035 38.473,36.16 31.961,40.152 30.364,41.132 28.79,41.988 27.365,42.594 25.902,43.217 24.768,43.5 24,43.5 c -0.768,0 -1.902,-0.283 -3.365,-0.906 -1.425,-0.606 -3,-1.463 -4.596,-2.442 C 9.527,36.16 5.623,29.035 5.51,21.33 Q 5.5,20.625 5.499,19.896 c 0,-5.091 0.402,-9.773 0.7,-12.532 C 6.307,6.356 6.832,5.838 7.437,5.703 M 24,0.5 C 14.52,0.5 9.336,1.179 6.565,1.798 3.921,2.39 2.47,4.63 2.222,6.935 1.916,9.776 1.5,14.611 1.5,19.895 q 0,0.76 0.011,1.493 c 0.132,9.013 4.704,17.434 12.437,22.174 1.691,1.037 3.45,2.002 5.121,2.713 1.634,0.695 3.359,1.225 4.931,1.225 1.572,0 3.297,-0.53 4.93,-1.225 1.672,-0.711 3.431,-1.676 5.122,-2.713 7.733,-4.74 12.305,-13.16 12.437,-22.174 q 0.01,-0.735 0.011,-1.492 C 46.5,14.611 46.084,9.776 45.778,6.935 45.53,4.63 44.079,2.389 41.435,1.798 38.663,1.178 33.48,0.5 24,0.5 m -8.693,10.804 c 1.7,-0.268 4.415,-0.51 8.693,-0.51 4.278,0 6.994,0.242 8.693,0.51 1.924,0.303 3.07,1.88 3.24,3.72 0.147,1.606 0.294,3.802 0.294,6.155 0,5.452 -2.766,10.596 -7.518,13.321 -0.84,0.482 -1.684,0.91 -2.477,1.218 -0.786,0.306 -1.555,0.509 -2.232,0.509 -0.677,0 -1.446,-0.203 -2.232,-0.509 -0.792,-0.308 -1.637,-0.736 -2.477,-1.218 -4.752,-2.725 -7.518,-7.87 -7.518,-13.321 0,-2.353 0.148,-4.55 0.295,-6.156 0.17,-1.838 1.316,-3.416 3.24,-3.719" clip-rule="evenodd" />
                    </svg>
                    <div>
                        <h3 class="font-serif text-xl font-bold text-primary">{t('dashboard.about_title')}</h3>
                    </div>
                </div>
                <button
                    onclick={() => showCreditModal = false}
                    class="rounded p-1 text-text-secondary transition-colors hover:text-primary"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                </button>
            </div>

            <p class="mb-6 font-sans text-sm leading-relaxed text-primary-soft">
                {t('dashboard.about_description')}
            </p>

            <div class="space-y-3 rounded-lg border border-border bg-surface-alt p-4">
                <div class="flex items-center justify-between">
                    <span class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.about_version')}</span>
                    <span class="font-mono text-sm text-primary">1.0.0</span>
                </div>
                <div class="border-t border-border"></div>
                <div class="flex items-center justify-between">
                    <span class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.about_author')}</span>
                    <span class="font-sans text-sm text-primary">Ajang Supardi</span>
                </div>
                <div class="border-t border-border"></div>
                <div class="flex items-center justify-between">
                    <span class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.about_email')}</span>
                    <a href="mailto:ajang@duck.com" class="font-mono text-sm text-teal underline underline-offset-2 hover:text-teal-light">ajang@duck.com</a>
                </div>
                <div class="border-t border-border"></div>
                <div class="flex items-center justify-between">
                    <span class="font-sans text-xs font-medium uppercase tracking-wide text-text-secondary">{t('dashboard.about_license')}</span>
                    <span class="font-sans text-sm text-primary">MIT License</span>
                </div>
            </div>

            <div class="mt-6 rounded-lg border border-teal bg-[#E8F4F8] p-4">
                <div class="flex items-start gap-3">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mt-0.5 shrink-0 text-error"><path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"/></svg>
                    <div>
                        <p class="font-serif text-sm font-bold text-primary">{t('dashboard.about_dedication')}</p>
                        <p class="mt-1 font-sans text-sm text-primary-soft">{t('dashboard.about_dedication_text')}</p>
                    </div>
                </div>
            </div>

            <div class="mt-6 flex justify-end">
                <button
                    onclick={() => showCreditModal = false}
                    class="rounded bg-gold px-6 py-2.5 font-serif text-sm font-normal text-black transition-colors hover:bg-gold-alt"
                >
                    OK
                </button>
            </div>
        </div>
    </div>
{/if}

{#if showSudoersModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div class="mx-4 w-full max-w-lg rounded-xl bg-surface p-6 shadow-floating">
            <div class="mb-4 flex items-center gap-3">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gold"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"/><path d="m9 12 2 2 4-4"/></svg>
                <h3 class="font-serif text-lg font-bold text-primary">{t('dashboard.sudoers_title')}</h3>
            </div>

            <p class="mb-3 font-sans text-sm text-primary-soft">
                {t('dashboard.sudoers_desc')}
            </p>

            {#if sudoersError}
                <div class="mb-4 rounded border border-error bg-[#FFEBEE] p-3">
                    <p class="mb-2 font-sans text-xs font-medium text-error">{t('dashboard.sudoers_error')}</p>
                    <pre class="overflow-x-auto rounded bg-surface p-2 font-mono text-xs text-primary">sudo tee /etc/sudoers.d/vpn-hizb &lt;&lt;'EOF'
vpn-hizb ALL=(root) NOPASSWD: /usr/sbin/openvpn
vpn-hizb ALL=(root) NOPASSWD: /usr/sbin/sysctl
vpn-hizb ALL=(root) NOPASSWD: /usr/bin/pkill openvpn
EOF
sudo chmod 440 /etc/sudoers.d/vpn-hizb</pre>
                    <button
                        onclick={handleSetupSudoers}
                        disabled={settingUpSudoers}
                        class="mt-3 rounded bg-teal px-4 py-1.5 font-serif text-xs font-normal text-white transition-colors hover:bg-teal-light"
                    >
                        {settingUpSudoers ? t('dashboard.sudoers_waiting') : t('dashboard.sudoers_retry')}
                    </button>
                </div>
            {/if}

            <div class="flex gap-3">
                {#if !sudoersError}
                    <button
                        onclick={handleSetupSudoers}
                        disabled={settingUpSudoers}
                        class="inline-flex items-center gap-2 rounded bg-gold px-5 py-2.5 font-serif text-sm font-normal text-black transition-colors hover:bg-gold-alt disabled:cursor-not-allowed disabled:bg-border-strong disabled:text-[#999999]"
                    >
                        {#if settingUpSudoers}
                            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="animate-spin"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>
                            {t('dashboard.sudoers_waiting')}
                        {:else}
                            {t('dashboard.sudoers_setup')}
                        {/if}
                    </button>
                {/if}
                <button
                    onclick={() => { showSudoersModal = false; sudoersError = ''; }}
                    class="rounded border border-primary bg-transparent px-5 py-2.5 font-serif text-sm font-normal text-primary transition-colors hover:bg-surface-alt"
                >
                    {sudoersError ? t('dashboard.close') : t('dashboard.sudoers_skip')}
                </button>
            </div>
        </div>
    </div>
{/if}
