use super::{
    get_or_create_device_id, normalize_base_url, poll_device_token, start_device_auth, AuthConfig,
};
use tauri::Emitter;

#[tauri::command]
pub async fn authenticate(app: tauri::AppHandle, server_url: String) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "Failed to get config directory".to_string())?
        .join("rommen");
    let base = normalize_base_url(&server_url)?;
    let device_id = get_or_create_device_id(&config_dir)?;
    let http = reqwest::Client::new();

    let (pairing, verify_url) = start_device_auth(&http, &base, &device_id).await?;

    // Open system default browser for RomM to handle the verification
    tauri_plugin_opener::open_url(verify_url.as_str(), None::<&str>).map_err(|e| {
        log::error!("Failed to open URL in browser: {e}");
        e.to_string()
    })?;

    let token = poll_device_token(&http, &base, &pairing).await?;

    let mut cfg = AuthConfig::load(&config_dir)?;
    cfg.server_url = base.to_string();
    cfg.access_token = Some(token.access_token);
    cfg.device_id = Some(token.device_id);
    cfg.scopes = token.scopes;
    cfg.expires_at = token.expires_at;
    cfg.save(&config_dir)?;

    app.emit("auth_success", ()).map_err(|e| {
        log::error!("Failed to emit auth success event: {e}");
        e.to_string()
    })?;
    Ok(())
}
