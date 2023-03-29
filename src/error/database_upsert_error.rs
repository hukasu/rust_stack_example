use error_stack::Context;

#[derive(Debug, Clone, Copy)]
pub struct DatabaseUpsertError;

impl std::fmt::Display for DatabaseUpsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to upsert values into database.")
    }
}

impl Context for DatabaseUpsertError {}