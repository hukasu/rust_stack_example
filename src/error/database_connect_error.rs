use error_stack::Context;

#[derive(Debug, Clone, Copy)]
pub struct DatabaseConnectError;

impl std::fmt::Display for DatabaseConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Connection to Postgres database has failed.")
    }
}

impl Context for DatabaseConnectError {}
