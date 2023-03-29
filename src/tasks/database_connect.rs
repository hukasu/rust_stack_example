use error_stack::{IntoReport, Result, ResultExt};

use crate::error::DatabaseConnectError;

/// Create a database connection from a `database_url` string.
pub async fn connect_to_database(database_url: &str) -> Result<sqlx::PgPool, DatabaseConnectError> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .into_report()
        .change_context(DatabaseConnectError)
        .attach("Failed to connect to Postgres database.")
}
