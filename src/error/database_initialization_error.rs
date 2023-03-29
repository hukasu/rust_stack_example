use error_stack::Context;

#[derive(Debug, Clone, Copy)]
pub struct DatabaseInitializationError;

impl std::fmt::Display for DatabaseInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to initialize database.")
    }
}

impl Context for DatabaseInitializationError {}
