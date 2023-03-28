use error_stack::{IntoReport, ResultExt, Result};
use axum::{Router, routing::get};
use tokio::sync::oneshot::Sender;

use crate::{error::ServerError, routes};

/// Initializes and runs Axum server.
/// Runs until a SIGTERM (or CTRL+C) is received.
pub async fn server_startup(database_pool: sqlx::PgPool, upsert_channel: Sender<()>) -> Result<(), ServerError> {
    log::trace!("Creating routers.");
    let api_router = Router::new()
        .route("/financial_data", get(routes::financial_data))
        .route("/statistics", get(routes::statistics));

    let app = Router::new().nest("/api", api_router)
        .layer(axum_sqlx_tx::Layer::new(database_pool));

    log::trace!("Binding server to port 8000.");
    let bind_url = "0.0.0.0:8000".parse().into_report()
        .change_context(ServerError)
        .attach("Failed to parse URL to bind to.")?;
    axum::Server::bind(&bind_url)
        .serve(app.into_make_service())
        .await
        .into_report()
        .change_context(ServerError)
        .attach("Failed to serve Axum server.")?;

    log::trace!("Sending signal to upsert task.");
    upsert_channel.send(()).map_err(|_| ServerError).into_report()
        .change_context(ServerError)
        .attach("Failed to send signal to Upsert task.")
}