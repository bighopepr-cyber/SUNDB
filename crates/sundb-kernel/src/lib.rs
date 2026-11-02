use sundb_core::{DatabaseConfig, Result};

#[derive(Debug, Clone)]
pub struct EngineRuntime {
    pub config: DatabaseConfig,
    pub started: bool,
}

impl EngineRuntime {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        if config.database_name.trim().is_empty() {
            return Err(sundb_core::SunDbError::InvalidConfiguration(
                "database name must not be empty".to_string(),
            ));
        }

        Ok(Self {
            config,
            started: true,
        })
    }

    pub fn status(&self) -> &'static str {
        if self.started { "running" } else { "stopped" }
    }
}
