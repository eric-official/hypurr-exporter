use crate::MAINNET_INFO_API_URL;
use anyhow::{Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum InfoRequest {
    #[serde(rename_all = "camelCase")]
    VaultDetails {
        vault_address: String,
    },
    Portfolio {
        user: String,
    },
    DelegatorSummary {
        user: String,
    },
    OpenOrders {
        user: String,
    },
}

pub async fn send_info_request<T: for<'a> Deserialize<'a>>(
    info_request: InfoRequest,
) -> anyhow::Result<T> {
    let http_client = Client::new();
    let url = MAINNET_INFO_API_URL.to_string();
    let data = serde_json::to_string(&info_request).context(format!(
        "Failed to deserialize the info_request {:?}",
        info_request
    ))?;

    let response = http_client
        .post(url.to_string())
        .header("Content-Type", "application/json")
        .body(data.clone())
        .send()
        .await?;
    debug!(
        "Received response with status {} for request {}",
        response.status(),
        data
    );

    let json_response = response
        .json()
        .await
        .context("Failed to deserialize the response body as JSON")?;

    Ok(json_response)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub user_address: Option<String>,
    pub vault_address: Option<String>,
}

pub async fn read_config() -> anyhow::Result<Config> {
    let content = std::fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&content)?)
}
