use serde::Serialize;

use super::{FinancialDataReport, Pagination, ResponseInfo};

/// Response returned from `financial_data` endpoint.
#[derive(Debug, Serialize)]
pub struct FinancialDataResponse {
    pub data: Vec<FinancialDataReport>,
    pub pagination: Pagination,
    pub info: ResponseInfo,
}
