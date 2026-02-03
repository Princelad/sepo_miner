use anyhow::{Context, Result};

/// Load the hCaptcha token from environment or file.
///
/// Resolution order:
/// 1) HCAPTCHA_TOKEN
/// 2) HCAPTCHA_TOKEN_FILE (file contents, trimmed)
pub fn load_hcaptcha_token() -> Result<String> {
    if let Ok(token) = std::env::var("HCAPTCHA_TOKEN") {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    if let Ok(path) = std::env::var("HCAPTCHA_TOKEN_FILE") {
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read HCAPTCHA_TOKEN_FILE at {}", path))?;
        let trimmed = contents.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    anyhow::bail!("Missing hCaptcha token. Set HCAPTCHA_TOKEN or HCAPTCHA_TOKEN_FILE.")
}
