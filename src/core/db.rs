use crate::adapters::AdapterSync;
use crate::core::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

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
