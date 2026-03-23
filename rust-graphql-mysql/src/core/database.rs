use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use dotenvy::dotenv;

pub async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
