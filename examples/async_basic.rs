use saberdb::{JsonFile, SaberDB};
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

#[tokio::main]
async fn main() -> saberdb::Result<()> {
    // Create adapter and database
    let adapter = JsonFile::new("db_async.json");
    let db = SaberDB::new(adapter, Database::default()).await?;

    println!("=== SaberDB Async Example ===\n");

    // CREATE: Add some posts
    println!("Creating posts...");
    {
        let mut data = db.data_mut().await;
        data.posts.push(Post {
            id: 1,
            title: "Hello Async SaberDB!".to_string(),
            views: 0,
        });
        data.posts.push(Post {
            id: 2,
            title: "Building a blazing fast async JSON database".to_string(),
            views: 150,
        });
    }
    db.write().await?;
    println!("Created {} posts\n", db.data().await.posts.len());

    // READ: Query posts
    println!("Reading posts...");
    {
        let data = db.data().await;
        for post in &data.posts {
            println!("  - [{}] {} (views: {})", post.id, post.title, post.views);
        }
    }
    println!();

    // UPDATE: Increment views
    println!("Updating post views...");
    db.update(|data| {
        if let Some(post) = data.posts.iter_mut().find(|p| p.id == 1) {
            post.views += 1;
        }
    })
    .await?;
    {
        let data = db.data().await;
        let post = data.posts.iter().find(|p| p.id == 1).unwrap();
        println!("Updated post 1 views to: {}\n", post.views);
    }

    // QUERY: Filter posts with high views
    println!("Filtering posts with views > 50...");
    {
        let data = db.data().await;
        let popular_posts: Vec<_> = data.posts.iter().filter(|p| p.views > 50).collect();
        println!("Found {} popular posts:", popular_posts.len());
        for post in popular_posts {
            println!("  - {}", post.title);
        }
    }
    println!();

    // DELETE: Remove a post
    println!("Deleting post 2...");
    db.update(|data| {
        data.posts.retain(|p| p.id != 2);
    })
    .await?;
    println!("Remaining posts: {}\n", db.data().await.posts.len());

    println!("=== Async example complete! Check db_async.json ===");

    Ok(())
}
