use serde::{Serialize, Deserialize};

/// Pagination information for list responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub count: usize,
    pub page: usize,
    pub limit: usize,
    pub pages: usize,
}
