mod json_file;

use async_trait::async_trait;
use crate::core::Result;

pub use json_file::{JsonFileSync, JsonFile};

/// Synchronous adapter trait for storage backends
pub trait AdapterSync<T>: Send + Sync {
    /// Read data from storage
    /// Returns None if the storage doesn't exist yet
    fn read(&self) -> Result<Option<T>>;

    /// Write data to storage
    fn write(&self, data: &T) -> Result<()>;
}

/// Asynchronous adapter trait for storage backends
#[async_trait]
pub trait Adapter<T>: Send + Sync {
    /// Read data from storage
    /// Returns None if the storage doesn't exist yet
    async fn read(&self) -> Result<Option<T>>;

    /// Write data to storage
    async fn write(&self, data: &T) -> Result<()>;
}
