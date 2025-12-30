use crate::core::Result;

/// Synchronous adapter trait for storage backends
pub trait AdapterSync<T>: Send + Sync {
    /// Read data from storage
    /// Returns None if the storage doesn't exist yet
    fn read(&self) -> Result<Option<T>>;

    /// Write data to storage
    fn write(&self, data: &T) -> Result<()>;
}
