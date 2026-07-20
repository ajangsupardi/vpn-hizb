use crate::services::sudoers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SudoersStatus {
    pub configured: bool,
}

#[tauri::command]
pub async fn check_sudoers() -> Result<SudoersStatus, String> {
    Ok(SudoersStatus {
        configured: sudoers::is_sudoers_configured(),
    })
}

#[tauri::command]
pub async fn setup_sudoers() -> Result<SudoersStatus, String> {
    sudoers::setup_sudoers()?;
    Ok(SudoersStatus {
        configured: sudoers::is_sudoers_configured(),
    })
}
