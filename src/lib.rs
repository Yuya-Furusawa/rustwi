mod constants {
    use std::env;

    pub const AXUM_SESSION_COOKIE_NAME: &str = "rustwi_session";
    pub const AXUM_SESSION_USER_ID_KEY: &str = "uid";

    pub fn database_url() -> String {
        dotenv::dotenv().ok();
        env::var("DATABASE_URL").unwrap()
    }
}

mod controllers {
    mod accounts;
    mod root;
    mod tweets;

    pub use accounts::accounts;
    pub use root::app;
    pub use tweets::tweets;
}

mod database;

mod entities {
    mod account;
    mod tweet;

    pub use account::Account;
    pub use tweet::Tweet;
}

mod repos_impl {
    mod accounts;
    mod tweets;

    pub use accounts::AccountsImpl;
    pub use tweets::TweetsImpl;
}

mod repositories {
    mod accounts;
    mod tweets;

    pub use accounts::Accounts;
    #[cfg(test)]
    pub use accounts::MockAccounts;
    #[cfg(test)]
    pub use tweets::MockTweets;
    pub use tweets::Tweets;
}

mod services {
    mod accounts;
    mod tweets;

    pub use accounts::{clear_session, create_account, create_session, SessionToken};
    pub use tweets::{create_tweet, delete_tweet, list_tweets};
}

mod request;

mod response;

mod views {
    mod home;
    mod sign_in;
    mod sign_up;
    mod partial {
        mod tweet;

        pub use tweet::Tweet;
    }

    pub use home::Home;
    pub use partial::Tweet;
    pub use sign_in::SignIn;
    pub use sign_up::SignUp;
}

pub use controllers::app;

pub async fn setup_session_store() {
    let database_url = constants::database_url();
    let store = async_sqlx_session::PostgresSessionStore::new(&database_url)
        .await
        .unwrap();
    store.migrate().await.unwrap();
    store.spawn_cleanup_task(std::time::Duration::from_secs(3600));
}
