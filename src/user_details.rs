use crate::{
    utils::{InfoRequest, send_info_request},
    vault_details::PortfolioEntry,
};
use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStakingSummary {
    delegated: String,
    undelegated: String,
    total_pending_withdrawal: String,
    n_pending_withdrawals: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrders {
    coin: String,
    limit_px: String,
    oid: i64,
    side: String,
    sz: String,
    timestamp: i64,
}

pub async fn get_user_details(
    user_address: String,
) -> anyhow::Result<(f64, f64, f64, f64, f64, usize, f64)> {
    let user_portfolio: Vec<PortfolioEntry> = send_info_request(InfoRequest::Portfolio {
        user: user_address.clone(),
    })
    .await?;

    let daily_portfolio_entries = if let Some(daily_portfolio_entries) =
        user_portfolio.iter().find(|entry| entry.period == "day")
    {
        daily_portfolio_entries
    } else {
        bail!("Couldn't find daily portfolio entries in the vault details!");
    };

    let latest_account_value = if let Some(latest_account_value) = daily_portfolio_entries
        .data
        .account_value_history
        .iter()
        .max_by_key(|entry| entry.0)
    {
        latest_account_value
    } else {
        bail!("Couldn't find find the latest account value of the vault!");
    };

    let latest_pnl = if let Some(latest_pnl) = daily_portfolio_entries
        .data
        .pnl_history
        .iter()
        .max_by_key(|entry| entry.0)
    {
        latest_pnl
    } else {
        bail!("Couldn't find find the latest PnL of the vault!");
    };

    let user_staking_summary: UserStakingSummary =
        send_info_request(InfoRequest::DelegatorSummary {
            user: user_address.clone(),
        })
        .await?;

    let user_open_orders: Vec<OpenOrders> =
        send_info_request(InfoRequest::OpenOrders { user: user_address }).await?;

    let user_account_value = latest_account_value.1.parse()?;
    let user_pnl = latest_pnl.1.parse()?;
    let user_staking_delegated = user_staking_summary.delegated.parse()?;
    let user_staking_undelegated = user_staking_summary.undelegated.parse()?;
    let user_staking_pending_withdrawal = user_staking_summary.total_pending_withdrawal.parse()?;
    let user_num_open_orders = user_open_orders.len();
    let user_value_open_orders = user_open_orders
        .iter()
        .filter_map(|order| {
            let px = order.limit_px.parse::<f64>().ok()?;
            let sz = order.sz.parse::<f64>().ok()?;
            Some(px * sz)
        })
        .sum();

    Ok((
        user_account_value,
        user_pnl,
        user_staking_delegated,
        user_staking_undelegated,
        user_staking_pending_withdrawal,
        user_num_open_orders,
        user_value_open_orders,
    ))
}
