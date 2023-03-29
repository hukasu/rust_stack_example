use serde::Serialize;
use sqlx::FromRow;

/// Single entry on the time series.
#[derive(Debug, Serialize, FromRow)]
pub struct FinancialDataReport {
    pub symbol: String,
    pub date: time::Date,
    pub open_price: f64,
    pub close_price: f64,
    pub volume: i32,
}
