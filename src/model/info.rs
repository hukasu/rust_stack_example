use serde::{Serialize, Deserialize};

/// Extra info for endpoint responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseInfo {
    pub error: String,
}
