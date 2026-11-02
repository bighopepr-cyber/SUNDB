use serde::{Deserialize, Serialize};
use sundb_core::{DatabaseConfig, Result, SunDbError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    SuperAdmin,
    DatabaseAdmin,
    Developer,
    ReadOnly,
    BackupOperator,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub username: String,
    pub role: Role,
    pub is_authenticated: bool,
}

#[derive(Debug, Clone)]
pub struct SecurityEngine {
    pub config: DatabaseConfig,
}

impl SecurityEngine {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<AuthContext> {
        if username.trim().is_empty() || password.trim().is_empty() {
            return Err(SunDbError::SecurityViolation(
                "username and password are required".to_string(),
            ));
        }

        Ok(AuthContext {
            username: username.to_string(),
            role: Role::ReadOnly,
            is_authenticated: true,
        })
    }
}
