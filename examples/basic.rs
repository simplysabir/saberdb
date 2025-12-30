use saberdb::{JsonFileSync, SaberDBSync};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    id: u32,
    title: String,
    views: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Database {
    posts: Vec<Post>,
}

fn main() -> saberdb::Result<()> {
    // Create adapter and database
    let adapter = JsonFileSync::new("db.json");
    let mut db = SaberDBSync::new(adapter, Database::default())?;

    println!("=== SaberDB Basic Example ===\n");

    // CREATE: Add some posts
    println!("Creating posts...");
    db.data_mut().posts.push(Post {
        id: 1,
        title: "Hello SaberDB!".to_string(),
        views: 0,
    });
    db.data_mut().posts.push(Post {
        id: 2,
        title: "Building a fast JSON database in Rust".to_string(),
        views: 100,
    });
    db.write()?;
    println!("Created {} posts\n", db.data().posts.len());

    // READ: Query posts
    println!("Reading posts...");
    for post in &db.data().posts {
        println!("  - [{}] {} (views: {})", post.id, post.title, post.views);
    }
    println!();

    // UPDATE: Increment views
    println!("Updating post views...");
    db.update(|data| {
        if let Some(post) = data.posts.iter_mut().find(|p| p.id == 1) {
            post.views += 1;
        }
    })?;
    println!("Updated post 1 views to: {}\n",
        db.data().posts.iter().find(|p| p.id == 1).unwrap().views);

    // QUERY: Filter posts with high views
    println!("Filtering posts with views > 50...");
    let popular_posts: Vec<_> = db.data()
        .posts
        .iter()
        .filter(|p| p.views > 50)
        .collect();
    println!("Found {} popular posts:", popular_posts.len());
    for post in popular_posts {
        println!("  - {}", post.title);
    }
    println!();

    // DELETE: Remove a post
    println!("Deleting post 2...");
    db.update(|data| {
        data.posts.retain(|p| p.id != 2);
    })?;
    println!("Remaining posts: {}\n", db.data().posts.len());

    println!("=== Example complete! Check db.json ===");

    Ok(())
}
