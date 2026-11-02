use sundb_core::{DatabaseConfig, Result};

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 7432,
            max_connections: 128,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerTransport {
    pub config: NetworkConfig,
    pub database: DatabaseConfig,
}

impl ServerTransport {
    pub fn new(config: NetworkConfig, database: DatabaseConfig) -> Result<Self> {
        Ok(Self { config, database })
    }

    pub async fn start(&self) -> Result<()> {
        let _ = (&self.config, &self.database);
        Ok(())
    }
}
