use crate::auth::{
    DeviceAuthInitRequest, DeviceAuthInitResponse, DeviceAuthTokenRequest, DeviceAuthTokenResponse,
};

pub async fn start_device_auth(
    http: &reqwest::Client,
    base: &url::Url,
    device_identifier: &str,
) -> Result<(DeviceAuthInitResponse, url::Url), String> {
    let init_url = base.join("api/auth/device/init").map_err(|e| {
        log::error!("Failed to join URL: {}", e);
        e.to_string()
    })?;

    let body = DeviceAuthInitRequest {
        client_device_identifier: device_identifier.to_string(),
        name: whoami::devicename().unwrap_or_else(|_| "Rommen".to_string()),
        client: "rommen".into(),
        platform: Some(whoami::platform().to_string()),
        client_version: Some(env!("CARGO_PKG_VERSION").to_string()),
        requested_scopes: vec![
            "platforms.read".into(),
            "roms.read".into(),
            "assets.read".into(),
            "me.read".into(),
        ],
    };

    let response = http.post(init_url).json(&body).send().await.map_err(|e| {
        log::error!("Failed to send device auth init request: {}", e);
        e.to_string()
    })?;

    if response.status() != reqwest::StatusCode::CREATED && !response.status().is_success() {
        let error = response.json::<serde_json::Value>().await.map_err(|e| {
            log::error!("Failed to parse device auth init response: {}", e);
            e.to_string()
        })?;
        return Err(format!(
            "Failed to start device auth: {}",
            error
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown error")
        ));
    }

    let pairing: DeviceAuthInitResponse = response.json().await.map_err(|e| {
        log::error!("Failed to parse device auth init response json: {}", e);
        e.to_string()
    })?;

    let verify = base
        .join(pairing.verification_path_complete.trim_start_matches('/'))
        .map_err(|e| {
            log::error!("Failed to join URL: {}", e);
            e.to_string()
        })?;

    Ok((pairing, verify))
}

pub async fn poll_device_token(
    http: &reqwest::Client,
    base: &url::Url,
    pairing: &DeviceAuthInitResponse,
) -> Result<DeviceAuthTokenResponse, String> {
    let token_url = base.join("api/auth/device/token").map_err(|e| {
        log::error!("Failed to join URL for polling device token: {}", e);
        e.to_string()
    })?;

    let deadline = std::time::Instant::now()
        + std::time::Duration::from_secs(std::cmp::max(1, pairing.expires_in));

    let mut interval = std::cmp::max(1, pairing.interval);

    while std::time::Instant::now() < deadline {
        let res = http
            .post(token_url.clone())
            .json(&DeviceAuthTokenRequest {
                device_code: pairing.device_code.clone(),
            })
            .send()
            .await
            .map_err(|e| {
                log::error!("Failed to send HTTP device token polling request: {}", e);
                e.to_string()
            })?;

        let status = res.status();
        let body = res.text().await.map_err(|e| {
            log::error!("Failed to parse device token polling response: {}", e);
            e.to_string()
        })?;

        if status.is_success() {
            return serde_json::from_str(&body).map_err(|e| {
                log::error!("Failed to parse device token polling response json: {}", e);
                e.to_string()
            })?;
        }

        if body.contains("access_denied") {
            return Err("Authorization Denied".into());
        }
        if body.contains("expired_token") || body.contains("expired") {
            return Err("authorization expired".into());
        }
        if body.contains("slow_down") {
            interval += 1;
        }
        // authorization_pending → keep polling
        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
    }
    Err("timed out waiting for approval".into())
}
