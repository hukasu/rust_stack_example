use error_stack::Context;

#[derive(Debug, Clone, Copy)]
pub struct ServerError;

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to run Axum server.")
    }
}

impl Context for ServerError {}
