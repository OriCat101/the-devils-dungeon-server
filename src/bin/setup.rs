use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", db_url);
    let pool = PgPool::connect(&db_url).await?;

    // Run migrations in migrations directory
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Migrations applied.");

    Ok(())
}
