use serde::{Deserialize, Serialize};
use sundb_core::{DatabaseConfig, Result, SunDbError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Active,
    Committed,
    RolledBack,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct WalEntry {
    pub transaction_id: u64,
    pub operation: String,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone)]
pub struct WalManager {
    pub config: DatabaseConfig,
    pub entries: Vec<WalEntry>,
}

impl WalManager {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        Ok(Self {
            config,
            entries: Vec::new(),
        })
    }

    pub fn append(&mut self, entry: WalEntry) {
        self.entries.push(entry);
    }
}

#[derive(Debug, Clone)]
pub struct TransactionManager {
    pub config: DatabaseConfig,
    pub current_status: TransactionStatus,
}

impl TransactionManager {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        Ok(Self {
            config,
            current_status: TransactionStatus::Active,
        })
    }

    pub fn begin(&mut self) -> Result<()> {
        self.current_status = TransactionStatus::Active;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<()> {
        if self.current_status == TransactionStatus::RolledBack {
            return Err(SunDbError::TransactionAborted(
                "cannot commit after rollback".to_string(),
            ));
        }

        self.current_status = TransactionStatus::Committed;
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<()> {
        self.current_status = TransactionStatus::RolledBack;
        Ok(())
    }
}
