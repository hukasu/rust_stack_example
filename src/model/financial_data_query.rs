use serde::Deserialize;

/// Values extracted from the URL query of the `financial_data` endpoint
#[derive(Debug, Deserialize)]
pub struct FinancialDataQuery {
    pub symbol: Option<String>,
    pub start_date: Option<time::Date>,
    pub end_date: Option<time::Date>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
