use sqlx::sqlite::SqlitePool;

pub async fn create_pool() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database.")
}
