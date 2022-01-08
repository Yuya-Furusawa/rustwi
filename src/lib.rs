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

    pub use accounts::{create_account, create_session};
    pub use tweets::{create_tweet, delete_tweet, list_tweets};
}

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
