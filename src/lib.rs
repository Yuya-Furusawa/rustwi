mod controllers {
    mod root;
    mod tweets;

    pub use root::app;
    pub use tweets::tweets;
}

mod database;

mod entities {
    mod tweet;

    pub use tweet::Tweet;
}

mod repos_impl {
    mod tweets;

    pub use tweets::TweetsImpl;
}

mod repositories {
    mod tweets;

    #[cfg(test)]
    pub use tweets::MockTweets;
    pub use tweets::Tweets;
}

mod services {
    mod tweets;

    pub use tweets::{create_tweet, list_tweets};
}

mod response;

mod views {
    mod home;
    mod partial {
        mod tweet;

        pub use tweet::Tweet;
    }

    pub use home::Home;
    pub use partial::Tweet;
}

pub use controllers::app;
