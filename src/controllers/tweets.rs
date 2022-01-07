use axum::{
    extract::{Extension, Form, Path},
    http::Uri,
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepositoryProvider;
use crate::services;

pub fn tweets() -> Router {
    Router::new()
        .route("/new", routing::post(post))
        .route("/:id/delete", routing::post(delete))
}

async fn post(
    form: Form<TweetForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let tweet_repo = repository_provider.tweets();
    services::create_tweet(&tweet_repo, &form.message).await;
    Redirect::to(Uri::from_static("/"))
}

async fn delete(
    Path(id): Path<i32>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let tweet_repo = repository_provider.tweets();
    services::delete_tweet(&tweet_repo, id).await;
    Redirect::to(Uri::from_static("/"))
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}
