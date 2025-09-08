use crate::strava::parsers::{Activity, Athlete};
use crate::schema::{ApiError, ApiResponse};
use crate::settings;
use crate::strava::client::{LoginUrl, StravaClient, TokenSet};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
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
    let state = Arc::new(AppState {
        strava_client_secret: client_secret.to_string(),
    });

    Router::new()
        .route("/login", get(handler_login_link))
        .route("/token_exchange", get(code_exchange_handler))
        .route("/token_refresh", get(token_refresh_handler))
        .route("/me", get(me_handler))
        .route("/activities", get(activity_handler))
        .with_state(state)
}

async fn handler_login_link(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<LoginUrl>, ApiError> {
    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    let link = sc.login_link().await;
    Ok(ApiResponse::JsonData(link))
}

#[derive(Serialize, Deserialize)]
struct CodeParams {
    state: String,
    code: String,
    scope: String,
}

async fn code_exchange_handler(
    State(state): State<Arc<AppState>>,
    Query(code_params): Query<CodeParams>,
) -> Result<ApiResponse<TokenSet>, ApiError> {
    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let token_set = match sc.code_exchange(&code_params.code).await {
        Ok(tokens) => tokens,
        Err(e) => return Err(reqwest_response_handling(e))
    };
    Ok(ApiResponse::JsonData(token_set))
}

async fn token_refresh_handler(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<TokenSet>, ApiError> {
    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let token_set = match sc.refresh_token().await {
        Ok(tokens) => tokens,
        Err(e) => return Err(reqwest_response_handling(e))
    };

    Ok(ApiResponse::JsonData(token_set))
}

fn reqwest_response_handling(
    error: reqwest::Error,
) -> ApiError {
        match error.status() {
            Some(StatusCode::UNAUTHORIZED) => ApiError {
                status_code: StatusCode::UNAUTHORIZED,
                message: "Something went wrong".to_string(),
            },
            _ => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error :D".to_string(),
            },
        }
}

async fn me_handler(State(state): State<Arc<AppState>>) -> Result<ApiResponse<Athlete>, ApiError> {
    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    let me = match sc.get_user().await {
        Ok(me) => me,
        Err(e) => return Err(reqwest_response_handling(e)),
    };

    Ok(ApiResponse::JsonData(me))
}

async fn activity_handler(
    State(state): State<Arc<AppState>>
) -> Result<ApiResponse<Vec<Activity>>, ApiError> {
    let sc = StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    let activities = match sc.get_activities().await {
        Ok(act) => act,
        Err(e) => return Err(reqwest_response_handling(e))
    };

    // Write activities to a file
    let _ = sc.write_activities(&activities.as_ref(), "./activities_file.json").await.unwrap();
    Ok(ApiResponse::JsonData(activities))
}
