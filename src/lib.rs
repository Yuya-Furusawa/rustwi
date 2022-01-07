mod controllers {
    mod root;
    mod tweets;

    pub use root::app;
    pub use tweets::tweets;
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
