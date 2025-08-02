use axum::{
    Router,
    body::Body,
    extract::State,
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use chrono::Utc;
use hypurr_exporter::{vault_details::get_vault_details, metrics::Metrics};
use prometheus::{Encoder, Registry, TextEncoder};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

#[derive(Clone)]
pub struct AppState {
    metrics: Arc<Mutex<Metrics>>,
    registry: Registry,
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

    let state = AppState { metrics, registry };

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
    let AppState { metrics, registry } = app_state;

    let vault_details = get_vault_details().await
    .unwrap_or_else(|e| {
        error!("Failed receive the vault details: {e:?}");
        (0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, false, false)
    });

    let metrics = metrics.lock().await;
    metrics
        .update(vault_details.0, vault_details.1, vault_details.2, vault_details.3, vault_details.4, vault_details.5, vault_details.6, vault_details.7, vault_details.8, vault_details.9)
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
