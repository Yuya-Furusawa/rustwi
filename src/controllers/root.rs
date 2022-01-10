use axum::{
    extract::{Extension, Query},
    response::{Headers, IntoResponse},
    routing, Router,
};
use serde::Deserialize;

use crate::controllers::{accounts, tweets};
use crate::database::{self, RepositoryProvider};
use crate::request::UserContext;
use crate::response;
use crate::services;
use crate::views::{SignIn, SignUp};

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .route("/login", routing::get(login))
        .route("/register", routing::get(register))
        .nest("/tweets", tweets::tweets())
        .nest("/accounts", accounts::accounts())
        .layer(database_layer)
}

async fn get(
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let tweet_repo = repository_provider.tweets();
    let account_repo = repository_provider.accounts();
    let home = services::list_tweets(&tweet_repo, &account_repo).await;
    response::from_template(home)
}

async fn login(query: Query<LoginQuery>) -> impl IntoResponse {
    let empty_session_token = services::clear_session();
    let headers = Headers(vec![("Set-Cookie", empty_session_token.cookie())]);
    let response = response::from_template(SignIn {
        error: query.error.is_some(),
    });
    (headers, response)
}

async fn register() -> impl IntoResponse {
    response::from_template(SignUp)
}

#[derive(Deserialize)]
struct LoginQuery {
    error: Option<String>,
}
