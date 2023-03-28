use axum::{Json, extract::Query};
use error_stack::{IntoReport, ResultExt};

use crate::{model::{ResponseInfo, StatisticsQuery, StatisticsReport, StatisticsResponse}, error::{ServerError, ResponseError}};

/// `statistics` endpoint.  
/// 
/// Returns the average opening price, closing price, and volume of a given global equity for a given date range.
/// 
/// # Query arguments
/// * `symbol` => Which global equity to query.
/// * `start_date` => Filters out dates earlier than this date.
/// * `end_date` => Filters out dates later than this date.
pub async fn statistics(
    mut db: axum_sqlx_tx::Tx<sqlx::Postgres>,
    Query(query): Query<StatisticsQuery>
) -> Result<Json<StatisticsResponse>, ResponseError<ServerError>> {
    log::trace!("Received request to `statistics`.");

    let query_str = 
    r#"
    SELECT *
    FROM (
        SELECT
            $1 as symbol,
            $2 as start_date,
            $3 as end_date,
            AVG(open_price) as average_daily_open_price,
            AVG(close_price) as average_daily_close_price,
            CAST(AVG(volume) as FLOAT8) as average_daily_volume
        FROM financial_data
        WHERE symbol = COALESCE($1, symbol) AND date BETWEEN COALESCE($2, date) AND COALESCE($3, date)
    ) as statistics
    WHERE average_daily_volume IS NOT NULL;
    "#;

    log::trace!("Querying statistics from database for a given global equity and date range.");
    let qresult = sqlx::query_as::<_, StatisticsReport>(query_str)
        .bind(&query.symbol)
        .bind(&query.start_date)
        .bind(&query.end_date)
        .fetch_optional(&mut db)
        .await.into_report()
        .change_context(ServerError)
        .attach("Failed to query financial data on Postgres database.")?;
    
    log::trace!("Verifying that a response from the database was returned and writing a matching response.");
    let msg = match &qresult {
        Some(_) => "".into(),
        None => "The query had no results. Try another date range and verify symbol is correct.".into()
    };

    log::trace!("Responding from `statistics` endpoint.");
    Ok(Json(
        StatisticsResponse {
            data: qresult,
            info: ResponseInfo { error : msg }
        }
    ))
}