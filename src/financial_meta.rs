use anyhow::bail;
use reqwest::Client;

use crate::consts::COINGECKO_HL_API_URL;

pub async fn get_coingecko_data(
    coingecko_key: &str,
) -> anyhow::Result<(f64, i64, i64, i64, f64, f64)> {
    let http_client = Client::new();
    let url = COINGECKO_HL_API_URL.to_string();
    let response = http_client
        .get(url)
        .header("accept", "application/json")
        .header("x-cg-api-key", coingecko_key)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;

    let hyperliquid_price =
        if let Some(hyperliquid_price) = json["market_data"]["current_price"]["usd"].as_f64() {
            hyperliquid_price
        } else {
            bail!("Failed to convert the price to f64!");
        };
    let hyperliquid_marketcap =
        if let Some(hyperliquid_marketcap) = json["market_data"]["market_cap"]["usd"].as_i64() {
            hyperliquid_marketcap
        } else {
            bail!("Failed to convert the marketcap to i64!");
        };
    let hyperliquid_fdv = if let Some(hyperliquid_fdv) =
        json["market_data"]["fully_diluted_valuation"]["usd"].as_i64()
    {
        hyperliquid_fdv
    } else {
        bail!("Failed to convert the fdv to i64!");
    };
    let hyperliquid_tvl =
        if let Some(hyperliquid_tvl) = json["market_data"]["total_value_locked"]["usd"].as_i64() {
            hyperliquid_tvl
        } else {
            bail!("Failed to convert the tvl to i64!");
        };
    let hyperliquid_circulating_supply = if let Some(hyperliquid_circulating_supply) =
        json["market_data"]["circulating_supply"].as_f64()
    {
        hyperliquid_circulating_supply
    } else {
        bail!("Failed to convert the circulating supply to f64!");
    };
    let hyperliquid_total_supply =
        if let Some(hyperliquid_total_supply) = json["market_data"]["total_supply"].as_f64() {
            hyperliquid_total_supply
        } else {
            bail!("Failed to convert the total supply to f64!");
        };

    Ok((
        hyperliquid_price,
        hyperliquid_marketcap,
        hyperliquid_fdv,
        hyperliquid_tvl,
        hyperliquid_circulating_supply,
        hyperliquid_total_supply,
    ))
}
