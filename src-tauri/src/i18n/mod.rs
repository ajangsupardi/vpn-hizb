pub mod en;
pub mod id;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub dashboard: HashMap<String, String>,
    pub server: HashMap<String, String>,
}

pub fn is_supported(locale: &str) -> bool {
    matches!(locale, "en" | "id")
}

pub fn get_translations(locale: &str) -> Translations {
    match locale {
        "id" => id::translations(),
        _ => en::translations(),
    }
}
