use tauri::State;

use crate::{
    error::KisaraResult,
    states::{config::KisaraConfig, ConfigState},
};

#[tauri::command]
pub async fn get_config(config: State<'_, ConfigState>) -> KisaraResult<KisaraConfig> {
    let config = config.lock().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn change_locale(
    config: State<'_, ConfigState>,
    locale: String,
) -> KisaraResult<KisaraConfig> {
    let mut config = config.lock().await;
    config.locale = locale;
    config.write_config()?;
    Ok(config.clone())
}
