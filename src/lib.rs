//! # SaberDB
//!
//! A blazingly fast, simple JSON database for Rust.
//!
//! ## Features
//!
//! - **Simple API** - Direct data manipulation, no query language needed
//! - **Type-safe** - Full Rust type safety with generics
//! - **Sync & Async** - Both synchronous and asynchronous APIs
//! - **Atomic writes** - Crash-safe with atomic file operations
//! - **Thread-safe** - True concurrent reads with async version
//!
//! ## Quick Start
//!
//! ### Synchronous API
//!
//! ```rust
//! use saberdb::{JsonFileSync, SaberDBSync};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize, Clone, Default)]
//! struct Database {
//!     counter: u32,
//! }
//!
//! # fn main() -> saberdb::Result<()> {
//! let adapter = JsonFileSync::new("db.json");
//! let mut db = SaberDBSync::new(adapter, Database::default())?;
//!
//! db.data_mut().counter = 42;
//! db.write()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Asynchronous API
//!
//! ```rust
//! use saberdb::{JsonFile, SaberDB};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize, Clone, Default)]
//! struct Database {
//!     counter: u32,
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> saberdb::Result<()> {
//! let adapter = JsonFile::new("db_async.json");
//! let db = SaberDB::new(adapter, Database::default()).await?;
//!
//! {
//!     let mut data = db.data_mut().await;
//!     data.counter = 42;
//! }
//! db.write().await?;
//! # Ok(())
//! # }
//! ```

pub mod adapters;
pub mod core;

pub use crate::core::{SaberDB, SaberDBSync, Result};
pub use crate::adapters::{Adapter, AdapterSync, JsonFile, JsonFileSync, Memory, MemorySync};
