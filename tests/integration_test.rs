use saberdb::{JsonFileSync, SaberDBSync};
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

#[test]
fn test_create_new_database() {
    let path = "test_create.json";
    cleanup(path);

    let adapter = JsonFileSync::new(path);
    let db = SaberDBSync::new(adapter, TestData::default()).unwrap();

    assert_eq!(db.data().counter, 0);
    assert_eq!(db.data().message, "default");

    cleanup(path);
}

#[test]
fn test_write_and_read() {
    let path = "test_write_read.json";
    cleanup(path);

    // Create and write
    {
        let adapter = JsonFileSync::new(path);
        let mut db = SaberDBSync::new(adapter, TestData::default()).unwrap();

        db.data_mut().counter = 42;
        db.data_mut().message = "hello saberdb".to_string();
        db.write().unwrap();
    }

    // Read back
    {
        let adapter = JsonFileSync::new(path);
        let db = SaberDBSync::new(adapter, TestData::default()).unwrap();

        assert_eq!(db.data().counter, 42);
        assert_eq!(db.data().message, "hello saberdb");
    }

    cleanup(path);
}

#[test]
fn test_update_atomically() {
    let path = "test_update.json";
    cleanup(path);

    let adapter = JsonFileSync::new(path);
    let mut db = SaberDBSync::new(adapter, TestData::default()).unwrap();

    // Update and write atomically
    db.update(|data| {
        data.counter = 100;
        data.message = "updated".to_string();
    })
    .unwrap();

    // Verify in memory
    assert_eq!(db.data().counter, 100);
    assert_eq!(db.data().message, "updated");

    // Verify on disk
    let adapter2 = JsonFileSync::new(path);
    let db2 = SaberDBSync::new(adapter2, TestData::default()).unwrap();
    assert_eq!(db2.data().counter, 100);

    cleanup(path);
}

#[test]
fn test_vec_operations() {
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    struct Posts {
        items: Vec<String>,
    }

    let path = "test_vec.json";
    cleanup(path);

    let adapter = JsonFileSync::new(path);
    let mut db = SaberDBSync::new(adapter, Posts::default()).unwrap();

    // Add items
    db.data_mut().items.push("first".to_string());
    db.data_mut().items.push("second".to_string());
    db.data_mut().items.push("third".to_string());
    db.write().unwrap();

    assert_eq!(db.data().items.len(), 3);

    // Filter
    let filtered: Vec<_> = db.data()
        .items
        .iter()
        .filter(|s| s.contains('i'))
        .collect();
    assert_eq!(filtered.len(), 2); // "first" and "third"

    // Remove
    db.update(|data| {
        data.items.retain(|s| s != "second");
    })
    .unwrap();

    assert_eq!(db.data().items.len(), 2);
    assert_eq!(db.data().items[0], "first");
    assert_eq!(db.data().items[1], "third");

    cleanup(path);
}

#[test]
fn test_persistence_across_instances() {
    let path = "test_persist.json";
    cleanup(path);

    // Instance 1: Create and write
    {
        let adapter = JsonFileSync::new(path);
        let mut db = SaberDBSync::new(adapter, TestData::default()).unwrap();
        db.data_mut().counter = 999;
        db.write().unwrap();
    }

    // Instance 2: Read
    {
        let adapter = JsonFileSync::new(path);
        let db = SaberDBSync::new(adapter, TestData::default()).unwrap();
        assert_eq!(db.data().counter, 999);
    }

    // Instance 3: Modify
    {
        let adapter = JsonFileSync::new(path);
        let mut db = SaberDBSync::new(adapter, TestData::default()).unwrap();
        assert_eq!(db.data().counter, 999);
        db.update(|data| data.counter += 1).unwrap();
    }

    // Instance 4: Verify
    {
        let adapter = JsonFileSync::new(path);
        let db = SaberDBSync::new(adapter, TestData::default()).unwrap();
        assert_eq!(db.data().counter, 1000);
    }

    cleanup(path);
}

#[test]
fn test_json_is_pretty_formatted() {
    let path = "test_pretty.json";
    cleanup(path);

    let adapter = JsonFileSync::new(path);
    let mut db = SaberDBSync::new(adapter, TestData::default()).unwrap();

    db.data_mut().counter = 42;
    db.data_mut().message = "test".to_string();
    db.write().unwrap();

    // Read raw file content
    let content = fs::read_to_string(path).unwrap();

    // Should be pretty-printed (contains newlines and indentation)
    assert!(content.contains('\n'));
    assert!(content.contains("  ")); // Has indentation

    cleanup(path);
}
