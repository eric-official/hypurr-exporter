use anyhow::bail;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    consts::ALCHEMY_API_URL,
    utils::{InfoRequest, send_info_request},
    vault_details::PortfolioEntry,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotMetaData {
    pub tokens: Vec<SpotToken>,
    pub universe: Vec<SpotUniversePair>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotToken {
    pub name: String,
    pub sz_decimals: u8,
    pub wei_decimals: u8,
    pub index: u32,
    pub token_id: String,
    pub is_canonical: bool,
    pub evm_contract: Option<String>,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotUniversePair {
    pub name: String,
    pub tokens: Vec<u32>,
    pub index: u32,
    pub is_canonical: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpMetaData {
    pub universe: Vec<PerpUniverseItem>,
    pub margin_tables: Vec<MarginTableEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpUniverseItem {
    pub name: String,
    pub sz_decimals: u8,
    pub max_leverage: u32,
    pub only_isolated: Option<bool>,
    pub is_delisted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTableEntry(pub u32, pub MarginTable);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTable {
    pub description: String,
    pub margin_tiers: Vec<MarginTier>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTier {
    pub lower_bound: String,
    pub max_leverage: u32,
}

pub async fn query_alchemy_api(http_client: Client, url: &str, body: Value) -> anyhow::Result<u64> {
    let response = http_client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;
    let json_response: Value = response.json().await?;

    let hex_result = if let Some(hex_result) = json_response["result"].as_str() {
        hex_result
    } else {
        bail!("Failed to extract hex result from the json response!");
    };
    let clean_hex = hex_result.trim_start_matches("0x");
    let decimal_result = match u64::from_str_radix(clean_hex, 16) {
        Ok(decimal_result) => decimal_result,
        Err(e) => {
            bail!("Invalid hex string for block number: {e}")
        }
    };

    Ok(decimal_result)
}

pub async fn get_protocol_data(alchemy_key: &str) -> anyhow::Result<(u64, u64, f64, usize, usize)> {
    let http_client = Client::new();
    let url = format!("{}{}", ALCHEMY_API_URL, alchemy_key);

    let body = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });
    let block_number = query_alchemy_api(http_client.clone(), &url, body).await?;

    let body = json!({
        "jsonrpc": "2.0",
        "method": "eth_gasPrice",
        "params": [],
        "id": 1
    });
    let base_fee = query_alchemy_api(http_client, &url, body).await?;

    let af_portfolio: Vec<PortfolioEntry> = send_info_request(InfoRequest::Portfolio {
        user: "0xfefefefefefefefefefefefefefefefefefefefe".to_string(),
    })
    .await?;

    let daily_portfolio_entries = if let Some(daily_portfolio_entries) =
        af_portfolio.iter().find(|entry| entry.period == "day")
    {
        daily_portfolio_entries
    } else {
        bail!("Couldn't find daily portfolio entries in the user details of the Assistance Fund!");
    };

    let latest_account_value = if let Some(latest_account_value) = daily_portfolio_entries
        .data
        .account_value_history
        .iter()
        .max_by_key(|entry| entry.0)
    {
        latest_account_value
    } else {
        bail!("Couldn't find find the latest account value of the Assistance Fund!");
    };

    let af_account_value = latest_account_value.1.parse()?;

    let spot_tokens: SpotMetaData = send_info_request(InfoRequest::SpotMeta).await?;
    let num_spot_tokens = spot_tokens.tokens.len();

    let perp_tokens: PerpMetaData = send_info_request(InfoRequest::Meta).await?;
    let num_perp_tokens = perp_tokens
        .universe
        .iter()
        .filter(|item| item.is_delisted != Some(true))
        .count();

    Ok((
        block_number,
        base_fee,
        af_account_value,
        num_spot_tokens,
        num_perp_tokens,
    ))
}
