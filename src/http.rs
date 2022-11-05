//! Webserver.

use axum::{routing::get, Router};

use crate::state::State;

/// Main axum router.
#[tracing::instrument(skip(state))]
pub fn app(state: State) -> Router<State> {
    Router::with_state(state).route("/", get(|| async { "Hello, World!" }))
}
