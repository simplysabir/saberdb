use async_trait::async_trait;
use crate::adapters::{Adapter, AdapterSync};
use crate::core::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// JSON file adapter for synchronous operations
pub struct JsonFileSync {
    path: PathBuf,
}

impl JsonFileSync {
    /// Create a new JSON file adapter
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl<T> AdapterSync<T> for JsonFileSync
where
    T: Serialize + DeserializeOwned,
{
    fn read(&self) -> Result<Option<T>> {
        match fs::read(&self.path) {
            Ok(bytes) => {
                let data = serde_json::from_slice(&bytes)?;
                Ok(Some(data))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn write(&self, data: &T) -> Result<()> {
        // Serialize to pretty JSON
        let json = serde_json::to_vec_pretty(data)?;

        // Atomic write: write to temp file, then rename
        let temp_path = self.path.with_extension("tmp");
        fs::write(&temp_path, json)?;
        fs::rename(temp_path, &self.path)?;

        Ok(())
    }
}

/// JSON file adapter for asynchronous operations
pub struct JsonFile {
    path: PathBuf,
}

impl JsonFile {
    /// Create a new async JSON file adapter
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl<T> Adapter<T> for JsonFile
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    async fn read(&self) -> Result<Option<T>> {
        match async_fs::read(&self.path).await {
            Ok(bytes) => {
                let data = serde_json::from_slice(&bytes)?;
                Ok(Some(data))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn write(&self, data: &T) -> Result<()> {
        // Serialize to pretty JSON
        let json = serde_json::to_vec_pretty(data)?;

        // Atomic write: write to temp file, then rename
        let temp_path = self.path.with_extension("tmp");
        async_fs::write(&temp_path, json).await?;
        async_fs::rename(temp_path, &self.path).await?;

        Ok(())
    }
}
