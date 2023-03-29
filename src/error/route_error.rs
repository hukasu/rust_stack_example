use axum::{http::status::StatusCode, response::IntoResponse};
use error_stack::Context;

#[derive(Debug, Clone, Copy)]
pub struct RouteError(pub &'static str);

impl std::fmt::Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The `{}` route has failed while processing request.",
            self.0
        )
    }
}

impl Context for RouteError {}

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
