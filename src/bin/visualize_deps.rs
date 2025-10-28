use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("##### Visualize the Dependency Graph! #####");
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Get all tasks
    let tasks = sqlx::query_as::<_, (Uuid, String)>("SELECT id, title FROM tasks ORDER BY title")
        .fetch_all(&pool)
        .await
        .unwrap();

    // Get all dependencies
    let deps =
        sqlx::query_as::<_, (Uuid, Uuid)>("SELECT task_id, depends_on FROM task_dependencies")
            .fetch_all(&pool)
            .await
            .unwrap();

    // Build adjacency list
    let mut graph: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for (task_id, depends_on) in deps {
        graph.entry(task_id).or_insert_with(Vec::new).push(depends_on);
    }

    // Task ID to a title map
    let task_map: HashMap<Uuid, String> = tasks.into_iter().collect();

    println!("📊 Task Dependency Graph:\n");

    for (task_id, title) in task_map.iter() {
        println!("Task: {}", title);

        if let Some(dependencies) = graph.get(task_id) {
            for dep_id in dependencies {
                if let Some(dep_title) = task_map.get(dep_id) {
                    println!("  └─ depends on: {}", dep_title);
                }
            }
        } else {
            println!("  └─ (no dependencies)");
        }
        println!();
    }
}
