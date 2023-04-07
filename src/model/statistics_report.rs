use serde::{Serialize, Deserialize};
use sqlx::FromRow;

/// Statistics from a global equity within a date range.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StatisticsReport {
    pub symbol: String,
    pub start_date: time::Date,
    pub end_date: time::Date,
    pub average_daily_open_price: f64,
    pub average_daily_close_price: f64,
    pub average_daily_volume: f64,
}
