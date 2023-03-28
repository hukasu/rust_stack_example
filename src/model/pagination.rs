use serde::Serialize;

/// Pagination information for list responses.
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub count: usize,
    pub page: usize,
    pub limit: usize,
    pub pages: usize
}