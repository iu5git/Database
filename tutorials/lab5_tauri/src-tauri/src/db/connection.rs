use sqlx::postgres::PgPool;
use std::env;
use tokio::sync::Mutex;

pub struct DbConnectionPool {
    pub connection: Mutex<PgPool>,
}

pub async fn establish_connection_pool() -> Result<DbConnectionPool, String> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url)
        .await
        .map_err(|e| format!("Error connecting to db: {}", e))?;

    Ok(DbConnectionPool {
        connection: Mutex::new(pool),
    })
}
