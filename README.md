# SaberDB

A blazingly fast, simple JSON database for Rust.

## Features

- **Simple API** - Direct data manipulation, no query language needed
- **Type-safe** - Full Rust type safety with generics
- **Sync & Async** - Both synchronous and asynchronous APIs
- **Atomic writes** - Crash-safe with atomic file operations
- **Thread-safe** - True concurrent reads with async version
- **Adapter pattern** - Extensible storage backends
- **Minimal dependencies** - Fast compilation
- **Pretty JSON** - Human-readable output

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
saberdb = "1.1"
```

## Quick Start

### Synchronous API

```rust
use saberdb::{JsonFileSync, SaberDBSync};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Database {
    posts: Vec<Post>,
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    id: u32,
    title: String,
    views: u32,
}

fn main() -> saberdb::Result<()> {
    // Create database
    let adapter = JsonFileSync::new("db.json");
    let mut db = SaberDBSync::new(adapter, Database::default())?;

    // Create
    db.data_mut().posts.push(Post {
        id: 1,
        title: "Hello SaberDB!".to_string(),
        views: 0,
    });
    db.write()?;

    // Read
    let post = db.data().posts.first().unwrap();
    println!("Post: {}", post.title);

    // Update
    db.update(|data| {
        if let Some(post) = data.posts.iter_mut().find(|p| p.id == 1) {
            post.views += 1;
        }
    })?;

    // Delete
    db.update(|data| {
        data.posts.retain(|p| p.id != 1);
    })?;

    Ok(())
}
```

### Asynchronous API

```rust
use saberdb::{JsonFile, SaberDB};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Database {
    posts: Vec<Post>,
}

#[tokio::main]
async fn main() -> saberdb::Result<()> {
    // Create async database
    let adapter = JsonFile::new("db.json");
    let db = SaberDB::new(adapter, Database::default()).await?;

    // Create
    {
        let mut data = db.data_mut().await;
        data.posts.push(Post {
            id: 1,
            title: "Hello Async!".to_string(),
            views: 0,
        });
    }
    db.write().await?;

    // Read with concurrent access
    let data = db.data().await;
    println!("Posts: {}", data.posts.len());

    // Update atomically
    db.update(|data| {
        data.posts[0].views += 1;
    }).await?;

    Ok(())
}
```

## Usage Guide

### Querying Data

Use native Rust iterators - no special query language needed:

```rust
// Filter
let popular: Vec<_> = db.data()
    .posts
    .iter()
    .filter(|p| p.views > 100)
    .collect();

// Find
let post = db.data()
    .posts
    .iter()
    .find(|p| p.id == 1);

// Sort
let mut sorted = db.data().posts.clone();
sorted.sort_by_key(|p| p.views);

// Map
let titles: Vec<_> = db.data()
    .posts
    .iter()
    .map(|p| &p.title)
    .collect();
```

### Updating Data

Two ways to update:

**1. Manual update + write:**
```rust
// Sync
db.data_mut().posts.push(new_post);
db.write()?;

// Async
{
    let mut data = db.data_mut().await;
    data.posts.push(new_post);
}
db.write().await?;
```

**2. Atomic update (recommended):**
```rust
// Sync
db.update(|data| {
    data.posts.push(new_post);
})?;

// Async
db.update(|data| {
    data.posts.push(new_post);
}).await?;
```

### Custom Adapters

Implement your own storage backend:

```rust
use saberdb::{AdapterSync, Result};
use serde::{Serialize, de::DeserializeOwned};

struct CustomAdapter;

impl<T> AdapterSync<T> for CustomAdapter
where
    T: Serialize + DeserializeOwned,
{
    fn read(&self) -> Result<Option<T>> {
        // Your read logic
        todo!()
    }

    fn write(&self, data: &T) -> Result<()> {
        // Your write logic
        todo!()
    }
}
```

## API Reference

### Core Types

- **`SaberDBSync<T, A>`** - Synchronous database
  - `new(adapter, default) -> Result<Self>` - Create new database
  - `data(&self) -> &T` - Get immutable reference
  - `data_mut(&mut self) -> &mut T` - Get mutable reference
  - `write(&self) -> Result<()>` - Write to storage
  - `update<F>(&mut self, f: F) -> Result<()>` - Update and write atomically

- **`SaberDB<T, A>`** - Asynchronous database
  - `new(adapter, default) -> Result<Self>` - Create new database
  - `data(&self) -> RwLockReadGuard<T>` - Get immutable reference
  - `data_mut(&self) -> RwLockWriteGuard<T>` - Get mutable reference
  - `write(&self) -> Result<()>` - Write to storage
  - `update<F>(&self, f: F) -> Result<()>` - Update and write atomically

### Adapters

- **`JsonFileSync`** - Sync JSON file adapter
- **`JsonFile`** - Async JSON file adapter
- **`MemorySync`** - Sync in-memory adapter (perfect for testing)
- **`Memory`** - Async in-memory adapter (perfect for testing)

### Traits

- **`AdapterSync<T>`** - Trait for sync storage backends
- **`Adapter<T>`** - Trait for async storage backends

## Examples

Check out the `examples/` directory:

- `basic.rs` - Synchronous CRUD operations
- `async_basic.rs` - Asynchronous CRUD operations
- `memory.rs` - Using in-memory adapters for testing

Run examples:

```bash
cargo run --example basic
cargo run --example async_basic
cargo run --example memory
```

## Testing

```bash
cargo test
```

## License

MIT

## Contributing

Contributions welcome! Please feel free to submit issues and pull requests.
