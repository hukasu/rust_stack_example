use serde::Deserialize;

/// Values extracted from the URL query of the `statistics` endpoint
#[derive(Debug, Deserialize)]
pub struct StatisticsQuery {
    pub symbol: String,
    pub start_date: time::Date,
    pub end_date: time::Date,
}
