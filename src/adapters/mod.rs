//! Storage adapters for different backends.

mod json_file;
mod memory;

use async_trait::async_trait;
use crate::core::Result;

pub use json_file::{JsonFileSync, JsonFile};
pub use memory::{MemorySync, Memory};

/// Synchronous adapter trait for storage backends.
///
/// Implement this trait to create custom storage backends for [`SaberDBSync`](crate::SaberDBSync).
///
/// # Example
///
/// ```rust
/// use saberdb::{AdapterSync, Result};
/// use serde::{Serialize, de::DeserializeOwned};
///
/// struct MyAdapter;
///
/// impl<T> AdapterSync<T> for MyAdapter
/// where
///     T: Serialize + DeserializeOwned,
/// {
///     fn read(&self) -> Result<Option<T>> {
///         // Custom read logic
///         Ok(None)
///     }
///
///     fn write(&self, data: &T) -> Result<()> {
///         // Custom write logic
///         Ok(())
///     }
/// }
/// ```
pub trait AdapterSync<T>: Send + Sync {
    /// Read data from storage.
    ///
    /// Returns `Ok(None)` if the storage doesn't exist yet (e.g., file not found).
    /// Returns `Ok(Some(data))` if data was successfully read.
    /// Returns `Err` if an error occurred.
    fn read(&self) -> Result<Option<T>>;

    /// Write data to storage.
    ///
    /// Should be atomic if possible to prevent data corruption.
    fn write(&self, data: &T) -> Result<()>;
}

/// Asynchronous adapter trait for storage backends.
///
/// Implement this trait to create custom async storage backends for [`SaberDB`](crate::SaberDB).
///
/// # Example
///
/// ```rust
/// use saberdb::{Adapter, Result};
/// use serde::{Serialize, de::DeserializeOwned};
/// use async_trait::async_trait;
///
/// struct MyAsyncAdapter;
///
/// #[async_trait]
/// impl<T> Adapter<T> for MyAsyncAdapter
/// where
///     T: Serialize + DeserializeOwned + Send + Sync,
/// {
///     async fn read(&self) -> Result<Option<T>> {
///         // Custom async read logic
///         Ok(None)
///     }
///
///     async fn write(&self, data: &T) -> Result<()> {
///         // Custom async write logic
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Adapter<T>: Send + Sync {
    /// Read data from storage asynchronously.
    ///
    /// Returns `Ok(None)` if the storage doesn't exist yet (e.g., file not found).
    /// Returns `Ok(Some(data))` if data was successfully read.
    /// Returns `Err` if an error occurred.
    async fn read(&self) -> Result<Option<T>>;

    /// Write data to storage asynchronously.
    ///
    /// Should be atomic if possible to prevent data corruption.
    async fn write(&self, data: &T) -> Result<()>;
}
