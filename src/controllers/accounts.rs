use axum::{
    extract::{Extension, Form},
    http::Uri,
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::database::RepositoryProvider;
use crate::services;

pub fn accounts() -> Router {
    Router::new()
        .route("/new", routing::post(post))
        .route("/session", routing::post(new_session))
}

async fn post(
    form: Form<SignUpForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> impl IntoResponse {
    let account_repo = repository_provider.accounts();
    services::create_account(
        &account_repo,
        &form.email,
        &form.password,
        &form.display_name,
    )
    .await;
    Redirect::to(Uri::from_static("/"))
}

async fn new_session(
    form: Form<SignInForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let account_repo = repository_provider.accounts();
    let is_signed_in = services::create_session(&account_repo, &form.email, &form.password).await;
    is_signed_in
        .then(|| Redirect::to(Uri::from_static("/")))
        .ok_or(Redirect::to(Uri::from_static("/login?error=invalid")))
}

#[derive(Deserialize)]
struct SignInForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct SignUpForm {
    email: String,
    password: String,
    display_name: String,
}
