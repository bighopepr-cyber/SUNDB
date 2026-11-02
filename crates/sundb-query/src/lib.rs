use sundb_core::{DatabaseConfig, Result, SunDbError};

#[derive(Debug, Clone)]
pub struct QueryEngine {
    pub config: DatabaseConfig,
}

impl QueryEngine {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn execute(&self, statement: &str) -> Result<String> {
        if statement.trim().is_empty() {
            return Err(SunDbError::InvalidConfiguration(
                "query statement must not be empty".to_string(),
            ));
        }

        Ok(format!("Query accepted: {}", statement.trim()))
    }
}
