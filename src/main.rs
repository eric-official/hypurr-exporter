use axum::{
    Router,
    body::Body,
    extract::State,
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use chrono::Utc;
use hypurr_exporter::{
    financial_meta::get_coingecko_data,
    metrics::Metrics,
    user_details::get_user_details,
    utils::{Config, read_config},
    vault_details::get_vault_details,
};
use prometheus::{Encoder, Registry, TextEncoder};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    metrics: Arc<Mutex<Metrics>>,
    registry: Registry,
    config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut labels = HashMap::new();
    labels.insert("timestamp".to_string(), Utc::now().to_string());

    let registry = Registry::new_custom(Some("hyperliquid".to_string()), Some(labels))?;
    let metrics = Metrics::new()?;
    metrics.register(&registry)?;
    let metrics = Arc::new(Mutex::new(metrics));

    let config = read_config().await?;
    info!(
        "Read config.toml with user address: {} and vault address: {}",
        config.clone().user_address.unwrap_or("None".into()),
        config.clone().vault_address.unwrap_or("None".into())
    );

    let state = AppState {
        metrics,
        registry,
        config,
    };

    let app = Router::new()
        .route("/metrics", get(handle_metrics))
        .with_state(state);

    let addr = String::from("0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Listening on {}.", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

pub async fn handle_metrics(
    State(app_state): State<AppState>,
) -> Result<Response, (StatusCode, String)> {
    let AppState {
        metrics,
        registry,
        config,
    } = app_state;

    let coingecko_financial_meta = if let Some(coingecko_key) = config.coingecko_key {
        info!("Querying financial meta information from Coingecko");
        get_coingecko_data(&coingecko_key)
            .await
            .unwrap_or_else(|e| {
                error!("Failed receive the vault details: {e:?}");
                (0.0, 0, 0, 0, 0.0, 0.0)
            })
    } else {
        info!(
            "No Coingecko key got configured. Skipping the query of financial meta information from Coingecko!"
        );
        (0.0, 0, 0, 0, 0.0, 0.0)
    };

    let vault_details = if let Some(vault_address) = config.vault_address {
        info!("Querying vault details for address: {}", vault_address);
        get_vault_details(&vault_address).await.unwrap_or_else(|e| {
            error!("Failed receive the vault details: {e:?}");
            (0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, false, false)
        })
    } else {
        info!("No vault address got configured. Skipping the query of vault details!");
        (0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, false, false)
    };

    let user_details = if let Some(user_address) = config.user_address {
        info!("Querying user details for address: {}", user_address);
        get_user_details(user_address).await.unwrap_or_else(|e| {
            error!("Failed receive the vault details: {e:?}");
            (0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0)
        })
    } else {
        info!("No user address got configured. Skipping the query of user details!");
        (0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0)
    };

    let metrics = metrics.lock().await;
    metrics
        .update(coingecko_financial_meta, vault_details, user_details)
        .map_err(|e| {
            let error_message = format!("Failed to update metrics: {e:?}");
            error!(error_message);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message)
        })?;

    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let encoded_metrics = encoder.encode_to_string(&metric_families).map_err(|e| {
        let error_message = format!("Failed to encode metrics: {e:?}");
        error!(error_message);
        (StatusCode::INTERNAL_SERVER_ERROR, error_message)
    })?;

    let response = Response::builder()
        .header(header::CONTENT_TYPE, encoder.format_type())
        .body(Body::from(encoded_metrics))
        .map_err(|e| {
            let error_message = format!("Failed to build response: {e:?}");
            error!(error_message);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message)
        })?;

    Ok(response)
}
