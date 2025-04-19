use tauri::Window;

use crate::error::KisaraResult;

#[tauri::command]
pub fn get_window_is_maximized(window: Window) -> KisaraResult<bool> {
    Ok(window.is_maximized()?)
}

#[tauri::command]
pub fn maximize_window(window: Window) -> KisaraResult<()> {
    window.maximize()?;
    Ok(())
}

#[tauri::command]
pub fn unmaximize_window(window: Window) -> KisaraResult<()> {
    window.unmaximize()?;
    Ok(())
}

#[tauri::command]
pub fn minimize_window(window: Window) -> KisaraResult<()> {
    window.minimize()?;
    Ok(())
}

#[tauri::command]
pub fn close_window(window: Window) -> KisaraResult<()> {
    window.close()?;
    Ok(())
}

#[tauri::command]
pub fn open_url(url: String) -> KisaraResult<()> {
    open::that_detached(url)?;
    Ok(())
}

#[tauri::command]
pub fn fullscreen_window(window: Window) -> KisaraResult<()> {
    window.set_fullscreen(true)?;
    Ok(())
}

#[tauri::command]
pub fn unfullscreen_window(window: Window) -> KisaraResult<()> {
    window.set_fullscreen(false)?;
    Ok(())
}
