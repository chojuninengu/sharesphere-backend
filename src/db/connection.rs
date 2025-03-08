use sqlx::sqlite::SqlitePool;
use std::env;
use dotenv::dotenv;

pub async fn establish_connection() -> SqlitePool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
