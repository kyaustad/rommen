use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DeviceAuthInitRequest {
    pub client_device_identifier: String,
    pub name: String,
    pub client: String,
    pub platform: Option<String>,
    pub client_version: Option<String>,
    pub requested_scopes: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct DeviceAuthInitResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_path: String,
    pub verification_path_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Serialize)]
pub struct DeviceAuthTokenRequest {
    pub device_code: String,
}

#[derive(Deserialize)]
pub struct DeviceAuthTokenResponse {
    pub access_token: String,
    pub device_id: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
}
