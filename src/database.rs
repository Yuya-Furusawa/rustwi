use crate::constants::database_url;
use axum::AddExtensionLayer;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::repos_impl::{AccountsImpl, TweetsImpl};

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> AddExtensionLayer<RepositoryProvider> {
    let manager = PostgresConnectionManager::new_from_stringlike(database_url(), NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    AddExtensionLayer::new(RepositoryProvider(pool))
}

#[derive(Clone)]
pub struct RepositoryProvider(ConnectionPool);

impl RepositoryProvider {
    pub fn tweets(&self) -> TweetsImpl {
        TweetsImpl { pool: &self.0 }
    }

    pub fn accounts(&self) -> AccountsImpl {
        AccountsImpl { pool: &self.0 }
    }
}
