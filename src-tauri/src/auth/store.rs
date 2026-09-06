use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub server_url: String,
    pub access_token: Option<String>,
    pub device_id: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
}

impl AuthConfig {
    fn path(config_dir: &std::path::Path) -> std::path::PathBuf {
        config_dir.join("auth-config.json")
    }

    pub fn load(config_dir: &std::path::Path) -> Result<Self, String> {
        let path = Self::path(config_dir);
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(path)
            .map_err(|e| log::error!(format!("Failed to read auth config: {e}")))?;
        serde_json::from_str(&contents)
            .map_err(|e| log::error!(format!("Failed to parse auth config: {e}")))?;
    }

    pub fn save(&self, config_dir: &std::path::Path) -> Result<(), String> {
        let path = Self::path(config_dir);
        let tmp = path.with_extension("json.tmp");
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| log::error!(format!("Failed to serialize auth config: {e}")))?;

        std::fs::write(&tmp, &contents)
            .map_err(|e| log::error!(format!("Failed to write auth config: {e}")))?;
        std::fs::rename(&tmp, &path)
            .map_err(|e| log::error!(format!("Failed to rename auth config: {e}")))?;
        Ok(())
    }
}
