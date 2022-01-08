use crate::entities::Account;
use crate::repositories::Accounts;

pub async fn create_account(repo: &impl Accounts, email: &str, password: &str, display_name: &str) {
    let new_account = Account::create(email, password, display_name);
    repo.store(&new_account).await;
}

pub async fn create_session(repo: &impl Accounts, email: &str, password: &str) -> bool {
    let account = repo.find_by(email).await;
    if let Some(account) = account {
        account.matches_password(password)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use crate::entities::Account;
    use crate::repositories::MockAccounts;

    fn account(id: i32) -> Account {
        Account::new(
            id,
            format!("{}@example.com", id),
            to_sha256(format!("password{}", id)),
            format!("display_name{}", id),
        )
    }

    fn to_sha256(str: String) -> String {
        let str = str.as_bytes();
        let hashed_str = Sha256::digest(str);
        format!("{:x}", hashed_str)
    }

    #[tokio::test]
    async fn test_create_account() {
        let mut accounts = MockAccounts::new();
        accounts
            .expect_store()
            .withf(move |e| {
                let account = account(1);
                e.email == account.email
                    && e.hashed_password == account.hashed_password
                    && e.display_name == account.display_name
            })
            .once()
            .return_const(());

        let account = account(1);
        super::create_account(
            &accounts,
            &account.email,
            "password1",
            &account.display_name,
        )
        .await;
    }

    #[tokio::test]
    async fn test_create_session() {
        let mut accounts = MockAccounts::new();
        accounts.expect_find_by().returning(|_| Some(account(1)));

        let account = account(1);
        let result = super::create_session(&accounts, &account.email, "password1").await;
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_create_session_not_found() {
        let mut accounts = MockAccounts::new();
        accounts.expect_find_by().returning(|_| None);

        let account = account(1);
        let result = super::create_session(&accounts, &account.email, "password1").await;
        assert_eq!(result, false);
    }
}
