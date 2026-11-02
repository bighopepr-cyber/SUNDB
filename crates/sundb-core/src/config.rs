use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_name: String,
    pub data_directory: String,
    pub wal_directory: String,
    pub page_size: usize,
    pub max_connections: usize,
    pub buffer_pool_mb: usize,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_name: "sundb".to_string(),
            data_directory: "./data".to_string(),
            wal_directory: "./data/wal".to_string(),
            page_size: 16 * 1024,
            max_connections: 128,
            buffer_pool_mb: 256,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseIdentity {
    pub uuid: String,
    pub name: String,
    pub version: String,
}

impl Default for DatabaseIdentity {
    fn default() -> Self {
        Self {
            uuid: "00000000-0000-0000-0000-000000000000".to_string(),
            name: "sundb".to_string(),
            version: crate::SUNDB_VERSION.to_string(),
        }
    }
}
