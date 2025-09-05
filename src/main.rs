mod endpoints;
mod ingester;
mod settings;
mod strava_client;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    let app = Router::new()
        .route("/login", get(handler_login_link))
        .route("/token_exchange", get(code_exchange_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3007")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handler_login_link() -> impl IntoResponse {
    // Load secret from environment
    // TODO: Move this .env reading to a shared state
    // TODO: Move this endpoints to a separate router
    dotenv::dotenv().ok();
    let mut settings = settings::Settings::new();

    settings
        .load("STRAVA_KEY")
        .expect("Could not load variable");
    let client_secret = settings.get_value("STRAVA_KEY").unwrap();

    let sc = strava_client::StravaClient::init("https://www.strava.com", client_secret.to_string());
    Json(sc.login_link().await)
}

#[derive(Serialize, Deserialize)]
struct CodeParams {
    state: String,
    code: String,
    scope: String,
}

async fn code_exchange_handler(Query(code_params): Query<CodeParams>) -> impl IntoResponse {
    // Load secret from environment
    // TODO: Move this .env reading to a shared state
    // TODO: Move this endpoints to a separate router
    dotenv::dotenv().ok();
    let mut settings = settings::Settings::new();

    settings
        .load("STRAVA_KEY")
        .expect("Could not load variable");
    let client_secret = settings.get_value("STRAVA_KEY").unwrap();

    let sc = strava_client::StravaClient::init("https://www.strava.com", client_secret.to_string());

    let token_set = sc
        .code_exchange(&code_params.code)
        .await
        .expect("Could not get token back");

    Json(token_set)
}
