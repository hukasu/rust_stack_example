use axum::{http::StatusCode, response::IntoResponse};
use error_stack::{Context, Report};

#[derive(Debug)]
pub struct ResponseError<T>(pub Report<T>) where T: Copy + IntoResponse + std::fmt::Display + std::marker::Sync + std::marker::Send + 'static;

impl<T> IntoResponse for ResponseError<T> where T: Copy + IntoResponse + std::fmt::Display + std::marker::Sync + std::marker::Send + 'static {
    fn into_response(self) -> axum::response::Response {
        log::error!("{:?}", self.0);
        let inner = *self.0.current_context();
        inner.into_response()
    }
}

impl<T> From<Report<T>> for ResponseError<T> where T: Copy + IntoResponse + std::fmt::Display + std::marker::Sync + std::marker::Send + 'static {
    fn from(value: Report<T>) -> Self {
        ResponseError(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GetRawDataError {
    Initialization,
    DatabaseConnect,
    DatabaseInit,
    AlphaVantageQuery,
    DatavaseUpsert
}

impl std::fmt::Display for GetRawDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            GetRawDataError::Initialization => "Failed during initialization.",
            GetRawDataError::DatabaseConnect => "Failed to connect to database.",
            GetRawDataError::DatabaseInit => "Failed to initialize database.",
            GetRawDataError::AlphaVantageQuery => "Failed to query Alpha Vantage.",
            GetRawDataError::DatavaseUpsert => "Failed to upsert values into database.",
        };
        writeln!(f, "{}", message)
    }
}

impl Context for GetRawDataError {}

#[derive(Debug, Clone, Copy)]
pub struct  ServerError;

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to run Axum server.")
    }
}

impl Context for ServerError {}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}