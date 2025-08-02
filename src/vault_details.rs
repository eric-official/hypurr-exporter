use serde::{Deserialize, Serialize};
use reqwest::{Client};
use tracing::{debug};
use anyhow::{bail, Context};
use crate::MAINNET_INFO_API_URL;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum InfoRequest {
    #[serde(rename_all = "camelCase")]
    VaultDetails {
        vault_address: String
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VaultDetails {
    pub name: String,
    pub vault_address: String,
    pub leader: String,
    pub description: String,
    pub portfolio: Vec<PortfolioEntry>,
    pub apr: f64,
    #[serde(default)]
    pub follower_state: Option<String>, 
    pub leader_fraction: f64,
    pub leader_commission: f64,
    pub followers: Vec<Follower>,
    pub max_distributable: f64,
    pub max_withdrawable: f64,
    pub is_closed: bool,
    pub relationship: Relationship,
    pub allow_deposits: bool,
    pub always_close_on_withdraw: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioEntry {
    pub period: String, 
    pub data: PortfolioData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioData {
    pub account_value_history: Vec<(u64, String)>, 
    pub pnl_history: Vec<(u64, String)>, 
    pub vlm: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub user: String,
    pub vault_equity: String,
    pub pnl: String,
    pub all_time_pnl: String,
    pub days_following: i64,
    pub vault_entry_time: u64,
    pub lockup_until: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Relationship {
    pub r#type: String, 
    pub data: RelationshipData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipData {
    pub child_addresses: Vec<String>,
}

#[derive(Debug)]
pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
}

async fn send_info_request<T: for<'a> Deserialize<'a>>(
        info_request: InfoRequest,
    ) -> anyhow::Result<T> {
        let http_client = Client::new();
        let url = MAINNET_INFO_API_URL.to_string();
        let data =
            serde_json::to_string(&info_request).context(format!("Failed to deserialize the info_request {:?}", info_request))?;

        let response = http_client.post(url.to_string()).header("Content-Type", "application/json").body(data.clone()).send().await?;
        debug!("Received response with status {} for request {}", response.status(), data);

        let json_response = response.json().await.context("Failed to deserialize the response body as JSON")?;

        Ok(json_response)
    }

pub async fn get_vault_details() -> anyhow::Result<(f64, f64, f64, f64, f64, usize, f64, f64, bool, bool)> {
    let vault_details: VaultDetails = send_info_request(InfoRequest::VaultDetails { vault_address: "0xdfc24b077bc1425ad1dea75bcb6f8158e10df303".to_string() }).await?;

    let daily_portfolio_entries = if let Some(daly_portfolio_entries) = vault_details.portfolio.iter().find(|entry| entry.period == "day") {
        daly_portfolio_entries
    } else {
        bail!("Couldn't find daily portfolio entries in the vault details!");
    };

    let latest_account_value = if let Some(latest_account_value) = daily_portfolio_entries.data.account_value_history.iter().max_by_key(|entry| entry.0) {
        latest_account_value
    } else {
        bail!("Couldn't find find the latest account value of the vault!");
    };

    let latest_pnl = if let Some(latest_pnl) = daily_portfolio_entries.data.pnl_history.iter().max_by_key(|entry| entry.0) {
        latest_pnl
    } else {
        bail!("Couldn't find find the latest PnL of the vault!");
    };

    let vault_value = latest_account_value.1.parse()?;
    let vault_pnl = latest_pnl.1.parse()?;
    let vault_apr = vault_details.apr;
    let vault_leader_fraction = vault_details.leader_fraction;
    let vault_leader_commission = vault_details.leader_commission;
    let vault_num_followers = vault_details.followers.len();
    let vault_max_distributable = vault_details.max_distributable;
    let vault_max_withdrawable = vault_details.max_withdrawable;
    let vault_is_closed = vault_details.is_closed;
    let vault_allow_deposits = vault_details.allow_deposits;

    let vault_details = (vault_value, vault_pnl, vault_apr, vault_leader_fraction, vault_leader_commission, vault_num_followers, vault_max_distributable, vault_max_withdrawable, vault_is_closed, vault_allow_deposits);

    Ok(vault_details)
}
