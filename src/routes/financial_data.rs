use axum::{Json, extract::Query};
use error_stack::{IntoReport, ResultExt};

use crate::{model::{FinancialDataResponse, FinancialDataReport, FinancialDataQuery, Pagination, ResponseInfo}, error::{ResponseError, RouteError}};

/// `financial_data` endpoint.  
/// 
/// Returns a list of entries of the time series for a global equity within a date range.
/// The number of entries per response can be passed with the `limit` query parameter.
/// The `page` query parameter can be used for pagination.
/// 
/// # Query arguments
/// * `symbol`: Optional => Which global equity to query. `None` for all equities.
/// * `start_date`: Optional => Filters out dates earlier than this date.
/// * `end_date`: Optional => Filters out dates later than this date.
/// * `limit`: Optional, Default=5 => Limits the number of entries per response.
/// * `page`: Optional, Default=1 => Page of the response, for when the number of entries is larger than the limit.
pub async fn financial_data(
    mut db: axum_sqlx_tx::Tx<sqlx::Postgres>,
    Query(query): Query<FinancialDataQuery>
) -> Result<Json<FinancialDataResponse>, ResponseError<RouteError>> {
    log::trace!("Received request to `financial_data`.");

    let query_str = 
    r#"
    SELECT *
    FROM financial_data
    WHERE symbol = COALESCE($1, symbol) AND date BETWEEN COALESCE($2, date) AND COALESCE($3, date)
    ORDER BY date DESC;
    "#;

    log::trace!("Querying time series entries from database for a given global equity and date range.");
    let qresult = sqlx::query_as::<_, FinancialDataReport>(query_str)
        .bind(query.symbol)
        .bind(query.start_date)
        .bind(query.end_date)
        .fetch_all(&mut db)
        .await.into_report()
        .change_context(RouteError("financial_data"))
        .attach("Failed to query financial data on PostgreSQL database.")?;

    log::trace!("Setting up variables for filtering.");
    let count = qresult.len();
    let limit = query.limit.unwrap_or(5);
    // client-side is 1-indexed, server-side is 0-indexed
    let page = match query.page.unwrap_or(1).checked_sub(1) {
        Some(p) => p,
        None => return Ok(Json(
            FinancialDataResponse {
                data: vec![],
                pagination: Pagination {
                    count: 0,
                    page: 0,
                    limit: 0,
                    pages: 0
                },
                info: ResponseInfo { error : "Page must be a positive number bigger than 0.".into() }
            }
        ))
    };
    let offset = limit * page;
    let pages = match count.checked_div(limit) {
        Some(p) => p,
        None => return Ok(Json(
            FinancialDataResponse {
                data: vec![],
                pagination: Pagination {
                    count: 0,
                    page: 0,
                    limit: 0,
                    pages: 0
                },
                info: ResponseInfo { error : "Limit must be a positive number bigger than 0.".into() }
            }
        ))
    };

    log::trace!("Verifying that a response from the database was returned and writing a matching response.");
    let msg = match count.cmp(&0) {
        std::cmp::Ordering::Greater => "".into(),
        _ => "The query had no results. Try another date range and verify symbol is correct.".into()
    };

    log::trace!("Filtering database response and trimming symbol.");
    let qres = qresult.into_iter().skip(offset).take(limit).map(
        |mut r| { r.symbol = r.symbol.trim().into(); r }
    ).collect();

    log::trace!("Responding from `financial_data` endpoint.");
    Ok(Json(
        FinancialDataResponse {
            data: qres,
            pagination: Pagination {
                count: count,
                page: page,
                limit: limit,
                pages: pages
            },
            info: ResponseInfo { error : msg }
        }
    ))
}