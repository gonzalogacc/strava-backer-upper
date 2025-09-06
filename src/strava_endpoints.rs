use crate::{settings, strava_client};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;


struct AppState {
    strava_client_secret: String,
}


pub fn strava_router() -> Router {
    // Load secret from environment
    dotenv::dotenv().ok();
    let mut settings = settings::Settings::new();

    settings
        .load("STRAVA_KEY")
        .expect("Could not load variable");

    // Set the secret in the shared state
    let client_secret = settings.get_value("STRAVA_KEY").unwrap();
    let state = Arc::new(AppState{strava_client_secret: client_secret.to_string()});

    Router::new()
        .route("/login", get(handler_login_link))
        .route("/token_exchange", get(code_exchange_handler))
        .with_state(state)
}

async fn handler_login_link(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let sc = strava_client::StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    Json(sc.login_link().await)
}

#[derive(Serialize, Deserialize)]
struct CodeParams {
    state: String,
    code: String,
    scope: String,
}

async fn code_exchange_handler(
    State(state): State<Arc<AppState>>,
    Query(code_params): Query<CodeParams>
) -> impl IntoResponse {
    let sc = strava_client::StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let token_set = sc
        .code_exchange(&code_params.code)
        .await
        .expect("Could not get token back");

    Json(token_set)
}
