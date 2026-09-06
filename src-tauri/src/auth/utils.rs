pub fn normalize_base_url(input: &str) -> Result<url::Url, String> {
    let trimmed = input.trim().trim_end_matches('/');

    let with_protocol = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    };

    let mut url =
        url::Url::parse(&with_protocol).map_err(|e| log::error!(format!("Invalid URL: {e}")))?;

    // Lastly ensure URL ends with trailing slash

    if !url.path().ends_with('/') {
        url.set_path(&format("{}/", url.path().trim_end_matches('/')));
    }

    Ok(url)
}
