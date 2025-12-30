use crate::adapters::{Adapter, AdapterSync};
use crate::core::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock as AsyncRwLock;

/// Synchronous database
pub struct SaberDBSync<T, A>
where
    T: Serialize + DeserializeOwned,
    A: AdapterSync<T>,
{
    adapter: Arc<A>,
    data: T,
}

impl<T, A> SaberDBSync<T, A>
where
    T: Serialize + DeserializeOwned,
    A: AdapterSync<T>,
{
    /// Create a new database instance
    ///
    /// If the adapter can read existing data, it will be loaded.
    /// Otherwise, the default value is used.
    pub fn new(adapter: A, default: T) -> Result<Self> {
        let data = match adapter.read()? {
            Some(d) => d,
            None => default,
        };

        Ok(Self {
            adapter: Arc::new(adapter),
            data,
        })
    }

    /// Get immutable reference to the data
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Get mutable reference to the data
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Write current data to storage
    pub fn write(&self) -> Result<()> {
        self.adapter.write(&self.data)
    }

    /// Update the data and write to storage atomically
    pub fn update<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.data);
        self.write()
    }
}

/// Asynchronous database
pub struct SaberDB<T, A>
where
    T: Serialize + DeserializeOwned + Send + Sync,
    A: Adapter<T>,
{
    adapter: Arc<A>,
    data: Arc<AsyncRwLock<T>>,
}

impl<T, A> SaberDB<T, A>
where
    T: Serialize + DeserializeOwned + Send + Sync + Clone,
    A: Adapter<T>,
{
    /// Create a new async database instance
    ///
    /// If the adapter can read existing data, it will be loaded.
    /// Otherwise, the default value is used.
    pub async fn new(adapter: A, default: T) -> Result<Self> {
        let data = match adapter.read().await? {
            Some(d) => d,
            None => default,
        };

        Ok(Self {
            adapter: Arc::new(adapter),
            data: Arc::new(AsyncRwLock::new(data)),
        })
    }

    /// Get immutable reference to the data
    pub async fn data(&self) -> tokio::sync::RwLockReadGuard<'_, T> {
        self.data.read().await
    }

    /// Get mutable reference to the data
    pub async fn data_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, T> {
        self.data.write().await
    }

    /// Write current data to storage
    pub async fn write(&self) -> Result<()> {
        let data = self.data.read().await;
        self.adapter.write(&*data).await
    }

    /// Update the data and write to storage atomically
    pub async fn update<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut T),
    {
        {
            let mut data = self.data.write().await;
            f(&mut data);
        }
        self.write().await
    }
}
