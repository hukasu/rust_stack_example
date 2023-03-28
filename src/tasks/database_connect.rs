use error_stack::{IntoReport, ResultExt, Result};

use crate::error::ServerError;

/// Create a database connection from a `database_url` string.
pub async fn connect_to_database(database_url: &str) -> Result<sqlx::PgPool, ServerError> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url).await
        .into_report()
        .change_context(ServerError)
        .attach("Failed to connect to Postgres database.")
}