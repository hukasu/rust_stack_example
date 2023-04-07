use serde::{Serialize, Deserialize};

use super::{ResponseInfo, StatisticsReport};

/// Type representing the response returned from `financial_data` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsResponse {
    pub data: Option<StatisticsReport>,
    pub info: ResponseInfo,
}
