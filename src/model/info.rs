use serde::Serialize;

/// Extra info for endpoint responses.
#[derive(Debug, Serialize)]
pub struct ResponseInfo {
    pub error: String
}