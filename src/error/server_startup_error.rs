use error_stack::Context;

#[derive(Debug, Clone, Copy)]
pub struct ServerStartupError;

impl std::fmt::Display for ServerStartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Axum server has encountered error while setting up.")
    }
}

impl Context for ServerStartupError {}
