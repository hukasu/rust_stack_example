use serde::Serialize;

use super::{ResponseInfo, StatisticsReport};

/// Type representing the response returned from `financial_data` endpoint.
#[derive(Debug, Serialize)]
pub struct StatisticsResponse {
    pub data: Option<StatisticsReport>,
    pub info: ResponseInfo
}