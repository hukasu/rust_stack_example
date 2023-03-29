mod database_connect_error;
pub use database_connect_error::*;
mod database_initialization_error;
pub use database_initialization_error::*;
mod database_upsert_error;
pub use database_upsert_error::*;
mod server_error;
pub use server_error::*;
mod server_startup_error;
pub use server_startup_error::*;
mod route_error;
pub use route_error::*;

use axum::response::IntoResponse;
use error_stack::Report;

#[derive(Debug)]
pub struct ResponseError<T>(pub Report<T>)
where
    T: IntoResponse
        + error_stack::Context
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + Copy
        + 'static;

impl<T> IntoResponse for ResponseError<T>
where
    T: IntoResponse
        + error_stack::Context
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + Copy
        + 'static,
{
    fn into_response(self) -> axum::response::Response {
        log::error!("{:?}", self.0);
        let inner = self.0.current_context();
        (*inner).into_response()
    }
}

impl<T> From<Report<T>> for ResponseError<T>
where
    T: IntoResponse
        + error_stack::Context
        + std::fmt::Display
        + std::marker::Sync
        + std::marker::Send
        + Copy
        + 'static,
{
    fn from(value: Report<T>) -> Self {
        ResponseError(value)
    }
}
