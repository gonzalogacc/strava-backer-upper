use crate::ingester::Athlete;
use crate::strava_client::{LoginUrl, TokenSet};
use crate::{settings, strava_client};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState {
    strava_client_secret: String,
}

enum ApiResponse<T> {
    OK,
    JsonData(T),
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

struct ApiError {
    status_code: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.message)).into_response()
    }
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
        .with_state(state)
}

async fn handler_login_link(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<LoginUrl>, ApiError> {
    let sc =
        strava_client::StravaClient::init("https://www.strava.com", &state.strava_client_secret);
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
        strava_client::StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let token_set = sc.code_exchange(&code_params.code).await;
    reqwest_response_handling(token_set)
}

async fn token_refresh_handler(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<TokenSet>, ApiError> {
    let sc =
        strava_client::StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    let token_set = sc.refresh_token().await;
    reqwest_response_handling(token_set)
}

fn reqwest_response_handling<T>(
    result: Result<T, reqwest::Error>,
) -> Result<ApiResponse<T>, ApiError> {
    match result {
        Ok(r) => Ok(ApiResponse::JsonData(r)),
        Err(e) => match e.status() {
            Some(StatusCode::UNAUTHORIZED) => Err(ApiError {
                status_code: StatusCode::UNAUTHORIZED,
                message: "Something went wrong".to_string(),
            }),
            _ => Err(ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error :D".to_string(),
            }),
        },
    }
}

async fn me_handler(State(state): State<Arc<AppState>>) -> Result<ApiResponse<Athlete>, ApiError> {
    let sc =
        strava_client::StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    let me = sc.get_user().await;
    reqwest_response_handling(me)
}
