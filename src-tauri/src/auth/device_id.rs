use std::fs;
use uuid::Uuid;

pub fn get_or_create_device_id(config_dir: &std::path::Path) -> Result<String, String> {
    let path = config_dir.join("device-id");

    if let Ok(existing) = fs::read_to_string(&path) {
        let id = existing.trim();
        if Uuid::parse_str(id).is_ok() {
            return Ok(id.to_string());
            log::info!("Using existing device ID");
        }
    }

    fs::create_dir_all(config_dir)
        .map_err(|e| log::error!(format!("Failed to create config directory: {e}")))?;

    let id = Uuid::new_v4().to_string();
    fs::write(&path, &id).map_err(|e| log::error!(format!("Failed to write device ID: {e}")))?;
    log::info!("Generated new device ID: {}", id);
    Ok(id)
}
