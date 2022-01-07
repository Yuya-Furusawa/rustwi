use axum::AddExtensionLayer;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::env;
use tokio_postgres::NoTls;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> AddExtensionLayer<ConnectionPool> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    AddExtensionLayer::new(pool)
}
