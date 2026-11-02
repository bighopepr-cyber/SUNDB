use serde::{Deserialize, Serialize};
use sundb_core::{DatabaseConfig, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeRole {
    Leader,
    Follower,
    Candidate,
}

#[derive(Debug, Clone)]
pub struct ClusterConfig {
    pub node_id: String,
    pub role: NodeRole,
    pub peers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClusterManager {
    pub config: DatabaseConfig,
    pub cluster: ClusterConfig,
}

impl ClusterManager {
    pub fn new(config: DatabaseConfig, cluster: ClusterConfig) -> Result<Self> {
        Ok(Self { config, cluster })
    }

    pub fn summary(&self) -> String {
        format!("node={} role={:?} peers={}", self.cluster.node_id, self.cluster.role, self.cluster.peers.len())
    }
}
