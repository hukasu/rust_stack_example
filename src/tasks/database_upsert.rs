use error_stack::{IntoReport, ResultExt, Result};
use serde::Deserialize;
use tokio::sync::oneshot::Receiver;

use crate::{model::FinancialDataReport, error::DatabaseUpsertError};

/// Generates a placeholder value for the `symbol` value for the `RawFinancialDataReport` struct.
fn default_resource() -> String {
    "uninitialized".into()
}

/// Values extracted from the CSV returned from the Alpha Vantage API.
#[derive(Debug, Deserialize)]
struct RawFinancialDataReport {
    #[serde(default = "default_resource")]
    pub symbol: String,
    pub timestamp: time::Date,
    pub open: f64,
    pub close: f64,
    pub volume: i32,
}

impl From<RawFinancialDataReport> for FinancialDataReport {
    /// Converts `RawFinancialDataReport` into `FinancialDataReport`.
    fn from(value: RawFinancialDataReport) -> Self {
        FinancialDataReport {
            symbol: value.symbol,
            date: value.timestamp,
            open_price: value.open,
            close_price: value.close,
            volume: value.volume
        }
    }
}

/// Queries the Alpha Vantange API for a given global equity.
async fn query_alpha_vantage(api_key: &str, symbol: &str) -> Result<Vec<FinancialDataReport>, DatabaseUpsertError> {
    log::trace!("Requesting data from Alpha Vantage API for `{}` in CSV format.", symbol);
    let resp = reqwest::get(
        format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&apikey={}&symbol={}&datatype=csv", api_key, symbol)
    ).await
        .into_report()
        .change_context(DatabaseUpsertError)
        .attach("Failed to query Alpha Vantage API.")?;
    log::trace!("Extracting text from response body.");
    let text = resp.text().await
        .into_report()
        .change_context(DatabaseUpsertError)
        .attach("Failed to read Alpha Vantage API query response body.")?;
    log::trace!("Create CSV reader.");
    let mut csv = csv::Reader::from_reader(text.as_bytes());

    log::trace!("Getting date from 2 weeks ago for filtering.");
    let today = time::OffsetDateTime::now_utc();
    let two_weeks = (today - time::Duration::weeks(2)).date();
    
    log::trace!("Deserializing CSV into `RawFinancialDataReport` objects, mapping them into `FinancialDataReport`, and returning.");
    csv.deserialize()
        .filter_map(
            |raw: std::result::Result<RawFinancialDataReport, csv::Error>| {
                match raw {
                    Ok(mut dr) => {
                        dr.symbol = symbol.to_string();
                        match &dr.timestamp.cmp(&two_weeks) {
                            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => Some(Ok(FinancialDataReport::from(dr))),
                            _ => None
                        }
                    },
                    Err(er) => Some(Err(er))
                }
            }
        )
        .collect::<std::result::Result<Vec<FinancialDataReport>, csv::Error>>()
        .into_report()
        .change_context(DatabaseUpsertError)
        .attach("Failed to process Alpha Vantage API response.")
}

/// Upserts `FinancialDataReport` into database.
async fn upsert_in_database(pool: sqlx::PgPool, rows: Vec<FinancialDataReport>) -> Result<(), DatabaseUpsertError> {
    let query = r#"
    INSERT INTO financial_data (symbol, date, open_price, close_price, volume)
    VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (symbol, date)
    DO UPDATE
    SET open_price = EXCLUDED.open_price, close_price = EXCLUDED.close_price, volume = EXCLUDED.volume;"#;
    log::trace!("Initializing upsert transaction.");
    let mut trans = pool.begin().await
        .into_report()
        .change_context(DatabaseUpsertError)
        .attach("Failed to create transaction on Postgres database.")?;
    
    log::trace!("Upserting each value from the Alpha Vantage API query into the database.");
    for r in rows.into_iter() {
        let urows = sqlx::query(query)
            .bind(r.symbol)
            .bind(r.date)
            .bind(r.open_price)
            .bind(r.close_price)
            .bind(r.volume)
            .execute(&mut trans).await
            .into_report()
            .change_context(DatabaseUpsertError)
            .attach("Failed to upsert value into database.")?;
        log::info!("`{}` rows were updated.", urows.rows_affected());
    }

    log::trace!("Committing upsert transaction.");
    trans.commit().await
        .into_report()
        .change_context(DatabaseUpsertError)
        .attach("Failed to commit transaction on Postgres database.")
}

/// Queries Alpha Vantage API and upserts into database
pub async fn get_raw_data(pool: sqlx::PgPool, api_key: String) -> Result<(), DatabaseUpsertError> {
    log::trace!("Querying AlphaVantage");
    let ibm = query_alpha_vantage(&api_key, "IBM").await?;
    let aapl = query_alpha_vantage(&api_key, "AAPL").await?;
    let rows = [ibm, aapl].into_iter()
        .flatten()
        .collect();
    
    log::trace!("Saving values into database");
    upsert_in_database(pool, rows).await
}

/// Creates a recurring task to collect data from Alpha Vantage and upserts into the database.
/// 
/// Runs every day. A channel is used signal if the task should be quit. The channel is queried every 5 seconds.
pub async fn recurring_get_raw_data(
    pool: sqlx::PgPool,
    mut stop_channel: Receiver<()>,
    api_key: String,
    days: i64
) -> Result<(), DatabaseUpsertError> {
    log::trace!("Collecting current time and initializing interval.");
    let mut last_exec = time::OffsetDateTime::now_utc();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    loop {
        let now = time::OffsetDateTime::now_utc();
        if now.cmp(&last_exec).is_ge() {
            log::trace!("Daily quering of Alpha Vantage API.");
            match get_raw_data(pool.clone(), api_key.clone()).await {
                Ok(_) => { last_exec += time::Duration::days(days); },
                Err(err) => { log::error!("{}", err); }
            };
        }
        
        log::trace!("Querying channel for stop signal.");
        let stop_signal = stop_channel.try_recv();
        match stop_signal {
            Ok(_) => break,
            Err(tokio::sync::oneshot::error::TryRecvError::Empty) => { interval.tick().await; },
            Err(tokio::sync::oneshot::error::TryRecvError::Closed) => return Err(DatabaseUpsertError).into_report()
                .attach("Recurring task was disconnected from it's channel"),  
        }
    }
    log::trace!("Exited from recurring task.");
    Ok(())
}