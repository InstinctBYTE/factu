use anyhow::{Context, Result};

use reqwest::Client;

use std::collections::HashMap;

use tracing::info;

use crate::config::HaciendaConfig;
use crate::models::TokenResponse;

pub async fn get_token(client: &Client, config: &HaciendaConfig) -> Result<TokenResponse> {
    info!("==================================================");
    info!("SOLICITANDO TOKEN OAUTH HACIENDA");
    info!("==================================================");

    let mut params = HashMap::new();

    params.insert("client_id", config.client_id.as_str());

    params.insert("grant_type", "password");

    params.insert("username", config.username.as_str());

    params.insert("password", config.password.as_str());

    let response = client
        .post(&config.token_url)
        .form(&params)
        .send()
        .await
        .context("Error enviando request OAuth")?;

    let status = response.status();

    info!("Status HTTP OAuth: {}", status);

    let body = response
        .text()
        .await
        .context("No se pudo leer body OAuth")?;

    info!("==================================================");
    info!("BODY RAW OAUTH");
    info!("==================================================");
    info!("{}", body);

    if !status.is_success() {
        anyhow::bail!("OAuth Hacienda fallo: {}", body);
    }

    let parsed: TokenResponse =
        serde_json::from_str(&body).context("Error parseando TokenResponse")?;

    info!("==================================================");
    info!("TOKEN OBTENIDO CORRECTAMENTE");
    info!("==================================================");

    info!("Tipo token: {}", parsed.token_type);

    info!("Expira en: {} segundos", parsed.expires_in);

    Ok(parsed)
}
