use saberdb::{JsonFile, SaberDB};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct TestData {
    counter: u32,
    message: String,
}

impl Default for TestData {
    fn default() -> Self {
        Self {
            counter: 0,
            message: "default".to_string(),
        }
    }
}

fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(format!("{}.tmp", path));
}

#[tokio::test]
async fn test_async_create_new_database() {
    let path = "test_async_create.json";
    cleanup(path);

    let adapter = JsonFile::new(path);
    let db = SaberDB::new(adapter, TestData::default()).await.unwrap();

    assert_eq!(db.data().await.counter, 0);
    assert_eq!(db.data().await.message, "default");

    cleanup(path);
}

#[tokio::test]
async fn test_async_write_and_read() {
    let path = "test_async_write_read.json";
    cleanup(path);

    // Create and write
    {
        let adapter = JsonFile::new(path);
        let db = SaberDB::new(adapter, TestData::default()).await.unwrap();

        {
            let mut data = db.data_mut().await;
            data.counter = 42;
            data.message = "hello async saberdb".to_string();
        }
        db.write().await.unwrap();
    }

    // Read back
    {
        let adapter = JsonFile::new(path);
        let db = SaberDB::new(adapter, TestData::default()).await.unwrap();

        assert_eq!(db.data().await.counter, 42);
        assert_eq!(db.data().await.message, "hello async saberdb");
    }

    cleanup(path);
}

#[tokio::test]
async fn test_async_update_atomically() {
    let path = "test_async_update.json";
    cleanup(path);

    let adapter = JsonFile::new(path);
    let db = SaberDB::new(adapter, TestData::default()).await.unwrap();

    // Update and write atomically
    db.update(|data| {
        data.counter = 100;
        data.message = "updated".to_string();
    })
    .await
    .unwrap();

    // Verify in memory
    assert_eq!(db.data().await.counter, 100);
    assert_eq!(db.data().await.message, "updated");

    // Verify on disk
    let adapter2 = JsonFile::new(path);
    let db2 = SaberDB::new(adapter2, TestData::default()).await.unwrap();
    assert_eq!(db2.data().await.counter, 100);

    cleanup(path);
}

#[tokio::test]
async fn test_async_concurrent_reads() {
    let path = "test_async_concurrent.json";
    cleanup(path);

    let adapter = JsonFile::new(path);
    let db = SaberDB::new(adapter, TestData::default()).await.unwrap();

    {
        let mut data = db.data_mut().await;
        data.counter = 999;
    }
    db.write().await.unwrap();

    // Multiple concurrent reads should work
    let (r1, r2, r3) = tokio::join!(db.data(), db.data(), db.data());

    assert_eq!(r1.counter, 999);
    assert_eq!(r2.counter, 999);
    assert_eq!(r3.counter, 999);

    cleanup(path);
}

#[tokio::test]
async fn test_async_persistence_across_instances() {
    let path = "test_async_persist.json";
    cleanup(path);

    // Instance 1: Create and write
    {
        let adapter = JsonFile::new(path);
        let db = SaberDB::new(adapter, TestData::default()).await.unwrap();
        {
            let mut data = db.data_mut().await;
            data.counter = 999;
        }
        db.write().await.unwrap();
    }

    // Instance 2: Read
    {
        let adapter = JsonFile::new(path);
        let db = SaberDB::new(adapter, TestData::default()).await.unwrap();
        assert_eq!(db.data().await.counter, 999);
    }

    // Instance 3: Modify
    {
        let adapter = JsonFile::new(path);
        let db = SaberDB::new(adapter, TestData::default()).await.unwrap();
        assert_eq!(db.data().await.counter, 999);
        db.update(|data| data.counter += 1).await.unwrap();
    }

    // Instance 4: Verify
    {
        let adapter = JsonFile::new(path);
        let db = SaberDB::new(adapter, TestData::default()).await.unwrap();
        assert_eq!(db.data().await.counter, 1000);
    }

    cleanup(path);
}
