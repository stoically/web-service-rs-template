//! Webserver.

use axum::{response::IntoResponse, routing::get, Router};

use crate::state::State;

/// Main axum router.
#[tracing::instrument(skip(state))]
pub fn app(state: State) -> Router<State> {
    Router::with_state(state).route("/", get(handler))
}

async fn handler(_state: axum::extract::State<State>) -> impl IntoResponse {
    "Hello, World!"
}
