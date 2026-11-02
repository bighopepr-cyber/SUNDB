use serde::{Deserialize, Serialize};
use sundb_core::{DatabaseConfig, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexType {
    BTree,
    Hash,
    Inverted,
    Vector,
    Graph,
}

#[derive(Debug, Clone)]
pub struct IndexService {
    pub config: DatabaseConfig,
    pub kind: IndexType,
}

impl IndexService {
    pub fn new(config: DatabaseConfig, kind: IndexType) -> Result<Self> {
        Ok(Self { config, kind })
    }

    pub fn kind_name(&self) -> &'static str {
        match self.kind {
            IndexType::BTree => "btree",
            IndexType::Hash => "hash",
            IndexType::Inverted => "inverted",
            IndexType::Vector => "vector",
            IndexType::Graph => "graph",
        }
    }
}
