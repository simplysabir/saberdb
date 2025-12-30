use saberdb::{Memory, MemorySync, SaberDB, SaberDBSync};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Counter {
    value: u32,
}

fn sync_example() -> saberdb::Result<()> {
    println!("=== Sync Memory Adapter Example ===\n");

    let adapter = MemorySync::new();
    let mut db = SaberDBSync::new(adapter.clone(), Counter::default())?;

    println!("Initial value: {}", db.data().value);

    db.data_mut().value = 42;
    db.write()?;
    println!("After update: {}", db.data().value);

    // Create another instance with the same adapter
    // Since they share the same Arc, they see the same data
    let db2 = SaberDBSync::new(adapter, Counter::default())?;
    println!("Second instance sees: {}", db2.data().value);

    Ok(())
}

async fn async_example() -> saberdb::Result<()> {
    println!("\n=== Async Memory Adapter Example ===\n");

    let adapter = Memory::new();
    let db = SaberDB::new(adapter.clone(), Counter::default()).await?;

    println!("Initial value: {}", db.data().await.value);

    {
        let mut data = db.data_mut().await;
        data.value = 100;
    }
    db.write().await?;
    println!("After update: {}", db.data().await.value);

    // Create another instance with the same adapter
    let db2 = SaberDB::new(adapter, Counter::default()).await?;
    println!("Second instance sees: {}", db2.data().await.value);

    Ok(())
}

#[tokio::main]
async fn main() -> saberdb::Result<()> {
    sync_example()?;
    async_example().await?;

    println!("\n=== Memory adapter is perfect for testing! ===");
    println!("- No file I/O overhead");
    println!("- Fast and ephemeral");
    println!("- Shared state via Arc");

    Ok(())
}
