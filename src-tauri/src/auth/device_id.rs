use std::fs;
use uuid::Uuid;

pub fn get_or_create_device_id(config_dir: &std::path::Path) -> Result<String, String> {
    let path = config_dir.join("device-id");

    if let Ok(existing) = fs::read_to_string(&path) {
        let id = existing.trim();
        if Uuid::parse_str(id).is_ok() {
            log::info!("Using existing device ID");
            return Ok(id.to_string());
        }
    }

    fs::create_dir_all(config_dir).map_err(|e| {
        log::error!("Failed to create config directory: {e}");
        e.to_string()
    })?;

    let id = Uuid::new_v4().to_string();
    fs::write(&path, &id).map_err(|e| {
        log::error!("Failed to write device ID: {e}");
        e.to_string()
    })?;
    log::info!("Generated new device ID: {id}");
    Ok(id)
}
