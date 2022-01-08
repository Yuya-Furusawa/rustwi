use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::Account;
use crate::repositories::Accounts;

pub struct AccountsImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Accounts for AccountsImpl<'a> {
    async fn find_by(&self, email: &str) -> Option<Account> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM accounts WHERE email = $1", &[&email])
            .await
            .unwrap();
        row.map(|r| r.into())
    }

    async fn store(&self, entity: &Account) {
        let conn = self.pool.get().await.unwrap();
        conn.execute(
            "INSERT INTO accounts (email, password, display_name) VALUES ($1, $2, $3)",
            &[&entity.email, &entity.hashed_password, &entity.display_name],
        )
        .await
        .ok();
    }
}

impl From<Row> for Account {
    fn from(r: Row) -> Self {
        Account::new(
            r.get("id"),
            r.get("email"),
            r.get("password"),
            r.get("display_name"),
        )
    }
}
