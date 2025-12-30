use async_trait::async_trait;
use crate::adapters::{Adapter, AdapterSync};
use crate::core::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::{Arc, RwLock};

/// In-memory adapter for synchronous operations.
///
/// Useful for testing or temporary storage without file I/O.
///
/// # Example
///
/// ```rust
/// use saberdb::{MemorySync, SaberDBSync};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// struct Data {
///     value: u32,
/// }
///
/// # fn main() -> saberdb::Result<()> {
/// let adapter = MemorySync::new();
/// let mut db = SaberDBSync::new(adapter, Data::default())?;
///
/// db.data_mut().value = 42;
/// db.write()?;
/// # Ok(())
/// # }
/// ```
pub struct MemorySync<T> {
    data: Arc<RwLock<Option<T>>>,
}

impl<T> MemorySync<T> {
    /// Create a new in-memory adapter.
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(None)),
        }
    }
}

impl<T> Default for MemorySync<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for MemorySync<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl<T> AdapterSync<T> for MemorySync<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync,
{
    fn read(&self) -> Result<Option<T>> {
        let guard = self.data.read().unwrap();
        Ok(guard.clone())
    }

    fn write(&self, data: &T) -> Result<()> {
        let mut guard = self.data.write().unwrap();
        *guard = Some(data.clone());
        Ok(())
    }
}

/// In-memory adapter for asynchronous operations.
///
/// Useful for testing or temporary storage without file I/O.
///
/// # Example
///
/// ```rust
/// use saberdb::{Memory, SaberDB};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// struct Data {
///     value: u32,
/// }
///
/// # #[tokio::main]
/// # async fn main() -> saberdb::Result<()> {
/// let adapter = Memory::new();
/// let db = SaberDB::new(adapter, Data::default()).await?;
///
/// {
///     let mut data = db.data_mut().await;
///     data.value = 42;
/// }
/// db.write().await?;
/// # Ok(())
/// # }
/// ```
pub struct Memory<T> {
    data: Arc<tokio::sync::RwLock<Option<T>>>,
}

impl<T> Memory<T> {
    /// Create a new in-memory adapter.
    pub fn new() -> Self {
        Self {
            data: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }
}

impl<T> Default for Memory<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Memory<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

#[async_trait]
impl<T> Adapter<T> for Memory<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync,
{
    async fn read(&self) -> Result<Option<T>> {
        let guard = self.data.read().await;
        Ok(guard.clone())
    }

    async fn write(&self, data: &T) -> Result<()> {
        let mut guard = self.data.write().await;
        *guard = Some(data.clone());
        Ok(())
    }
}
