use std::collections::{HashMap, HashSet};

use crate::entities::Account;

#[cfg_attr(test, mockall::automock)]
#[axum::async_trait]
pub trait Accounts {
    async fn find(&self, ids: HashSet<i32>) -> HashMap<i32, Account>;
    async fn find_by(&self, email: &str) -> Option<Account>;
    async fn store(&self, entity: &Account);
}
