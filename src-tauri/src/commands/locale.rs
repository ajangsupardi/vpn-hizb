use crate::i18n;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

pub struct LocaleState {
    pub current: Mutex<String>,
}

impl LocaleState {
    pub fn new() -> Self {
        Self {
            current: Mutex::new("en".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LocaleResponse {
    pub locale: String,
    pub translations: i18n::Translations,
}

#[tauri::command]
pub async fn set_locale(
    locale: String,
    state: State<'_, LocaleState>,
) -> Result<LocaleResponse, String> {
    if !i18n::is_supported(&locale) {
        return Err("Unsupported locale".to_string());
    }
    *state.current.lock().unwrap() = locale.clone();
    let translations = i18n::get_translations(&locale);
    Ok(LocaleResponse {
        locale,
        translations,
    })
}

#[tauri::command]
pub async fn get_translations(locale: String) -> Result<i18n::Translations, String> {
    Ok(i18n::get_translations(&locale))
}
