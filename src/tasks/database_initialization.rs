use error_stack::{IntoReport, Result, ResultExt};

use crate::error::DatabaseInitializationError;

/// Creates the `financial_data` table into the database from the schema file if not exists.
pub async fn create_table_if_not_exists(
    pool: sqlx::PgPool,
) -> Result<(), DatabaseInitializationError> {
    log::trace!("Reading schema file.");
    let schema = std::fs::read_to_string("schema.sql")
        .or_else(|_| std::fs::read_to_string("/var/app/schema.sql"))
        .into_report()
        .change_context(DatabaseInitializationError)
        .attach("Failed to read schema file.")?;

    log::trace!("Starting create table transaction.");
    let mut trans = pool
        .begin()
        .await
        .into_report()
        .change_context(DatabaseInitializationError)
        .attach("Failed to create transaction on Postgres database.")?;

    log::trace!("Creating table if not exists.");
    let rows = sqlx::query(&schema)
        .execute(&mut trans)
        .await
        .into_report()
        .change_context(DatabaseInitializationError)
        .attach("Failed to create table on Postgres database.")?;
    log::info!("`{}` tables were updated.", rows.rows_affected());

    log::trace!("Committing create table transaction.");
    trans
        .commit()
        .await
        .into_report()
        .change_context(DatabaseInitializationError)
        .attach("Failed to commit transaction on Postgres database.")
}
