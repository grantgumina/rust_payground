use std::env;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
pub async fn connect() -> Result<(), sqlx::Error> {
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //let db_url: String = env::var("DATABASE_URL");
    let _pool: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(&db_url).await?;
    println!("Connected to database");
    Ok(())
}