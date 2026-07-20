use crate::services::vpngate::VpngateApiService;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct RefreshState {
    pub running: Arc<Mutex<bool>>,
    pub last_success: Arc<Mutex<Option<u64>>>,
}

impl RefreshState {
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            last_success: Arc::new(Mutex::new(None)),
        }
    }

    pub fn clone_inner(&self) -> (Arc<Mutex<bool>>, Arc<Mutex<Option<u64>>>) {
        (self.running.clone(), self.last_success.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshStatus {
    pub running: bool,
    pub last_success: Option<u64>,
}

#[tauri::command]
pub async fn refresh_servers(
    state: State<'_, AppState>,
    refresh_state: State<'_, RefreshState>,
) -> Result<RefreshStatus, String> {
    {
        let mut running = refresh_state.running.lock().unwrap();
        if *running {
            return Ok(RefreshStatus {
                running: true,
                last_success: *refresh_state.last_success.lock().unwrap(),
            });
        }
        *running = true;
    }

    let db = state.db.clone();
    let (running, last_success) = refresh_state.clone_inner();

    std::thread::spawn(move || {
        let api = VpngateApiService::new();
        match api.fetch_servers() {
            Ok(servers) => {
                let _ = db.save_servers(&servers);
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                *last_success.lock().unwrap() = Some(now);
            }
            Err(e) => {
                log::error!("Failed to refresh servers: {}", e);
            }
        }
        *running.lock().unwrap() = false;
    });

    Ok(RefreshStatus {
        running: true,
        last_success: *refresh_state.last_success.lock().unwrap(),
    })
}

#[tauri::command]
pub async fn get_refresh_status(refresh_state: State<'_, RefreshState>) -> Result<RefreshStatus, String> {
    Ok(RefreshStatus {
        running: *refresh_state.running.lock().unwrap(),
        last_success: *refresh_state.last_success.lock().unwrap(),
    })
}
