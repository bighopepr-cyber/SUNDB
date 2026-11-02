use sundb_core::{DatabaseConfig, Result};

#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub model_name: String,
    pub dimension: usize,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct AiEngine {
    pub config: DatabaseConfig,
    pub embedding: EmbeddingConfig,
}

impl AiEngine {
    pub fn new(config: DatabaseConfig, embedding: EmbeddingConfig) -> Result<Self> {
        Ok(Self { config, embedding })
    }

    pub fn build_summary(&self) -> String {
        format!("AI enabled={} model={} dim={}", self.embedding.enabled, self.embedding.model_name, self.embedding.dimension)
    }
}
