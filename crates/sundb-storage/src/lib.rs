use serde::{Deserialize, Serialize};
use sundb_core::{DatabaseConfig, Result, SunDbError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageType {
    Data,
    Index,
    Wal,
    Catalog,
    Metadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub id: PageId,
    pub page_type: PageType,
    pub size: usize,
    pub checksum: u32,
}

#[derive(Debug, Clone)]
pub struct BufferPool {
    pub config: DatabaseConfig,
    pub pages: Vec<Page>,
}

impl BufferPool {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        Ok(Self {
            config,
            pages: Vec::new(),
        })
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn total_pages(&self) -> usize {
        self.pages.len()
    }
}

#[derive(Debug, Clone)]
pub struct StorageEngine {
    pub config: DatabaseConfig,
}

impl StorageEngine {
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        if config.page_size == 0 {
            return Err(SunDbError::InvalidConfiguration(
                "page size must be greater than zero".to_string(),
            ));
        }

        Ok(Self { config })
    }

    pub fn create_page(&self, page_id: u64, page_type: PageType) -> Page {
        Page {
            id: PageId(page_id),
            page_type,
            size: self.config.page_size,
            checksum: 0,
        }
    }
}
