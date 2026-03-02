use crate::strava::parsers::{Activity, Athlete};
use crate::strava::client::{LoginUrl, StravaClient, TokenSet};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{ApiError, ApiResponse};
use diesel::prelude::*;
use crate::models::athlete::{AthleteRow, NewAthleteRow, create_athlete};
use crate::settings;

use chrono::Utc;
use deadpool_diesel::postgres::Pool; // Import the Pool type

#[derive(Clone)]
struct StravaState {
    strava_client_secret: String,
    conn: Pool
}


pub fn strava_router(conn: Pool) -> Router {
    // Load secret from environment
    let mut settings = settings::Settings::new();
    settings
        .load("STRAVA_KEY")
        .expect("Could not load variable");
    let client_secret: &String = settings.get_value("STRAVA_KEY").unwrap();

    let strava_state = Arc::new(StravaState {
        strava_client_secret: client_secret.to_string(),
        conn
    });
    
    Router::new()
        .route("/login", get(handler_login_link))
        .route("/token_exchange", get(code_exchange_handler))
        .route("/token_refresh", get(token_refresh_handler))
        .route("/me", get(me_handler))
        .route("/activities", get(activity_handler)).with_state(strava_state)
}

async fn handler_login_link(
    State(state): State<Arc<StravaState>>,
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
    State(state): State<Arc<StravaState>>,
    Query(code_params): Query<CodeParams>,
) -> Result<ApiResponse<TokenSet>, ApiError> {
    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let token_set = match sc.code_exchange(&code_params.code).await {
        Ok(tokens) => tokens,
        Err(e) => return Err(error_handling(e))
    };
    Ok(ApiResponse::JsonData(token_set))
}

async fn token_refresh_handler(
    State(state): State<Arc<StravaState>>,
) -> Result<ApiResponse<TokenSet>, ApiError> {
    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let token_set = match sc.refresh_token().await {
        Ok(tokens) => tokens,
        Err(e) => return Err(error_handling(e))
    };

    Ok(ApiResponse::JsonData(token_set))
}

fn error_handling(
    error: reqwest::Error,
) -> ApiError {
    // Convert to a handled error
    match error.status() {
        Some(StatusCode::UNAUTHORIZED) => ApiError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Unathorized, get or refresh the token".to_string(),
        },
        Some(StatusCode::BAD_REQUEST) => ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Bad request".to_string(),
        },
        _ => ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error :D".to_string(),
},
    }
}

async fn me_handler(State(state): State<Arc<StravaState>>) -> Result<ApiResponse<Athlete>, ApiError> {
    use crate::schema::athletes::dsl::*;

    let sc =
        StravaClient::init("https://www.strava.com", &state.strava_client_secret);

    let me = match sc.get_user().await {
        Ok(me) => me,
        Err(e) => return Err(error_handling(e)),
    };
    println!("lalalal");
    let conn = state.conn.get().await.expect("Connection not found");
    println!("After connA");
    let response = create_athlete(
        conn,
        me.id,
        me.clone().to_string(),
        me.firstname.clone(),
        me.lastname.clone(),
    ).await;
    match response {
        Ok(_) => Ok(ApiResponse::JsonData(me)),
        Err(e) => return  Err(ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: String::from("Something went wrong")})
    }


}

async fn activity_handler(
    State(state): State<Arc<StravaState>>
) -> Result<ApiResponse<Vec<Activity>>, ApiError> {
    let sc = StravaClient::init("https://www.strava.com", &state.strava_client_secret);
    let activities = match sc.get_activities().await {
        Ok(act) => act,
        Err(e) => return Err(error_handling(e))
    };

    // Write activities to a file
    let _ = sc.write_activities(&activities.as_ref(), "./activities_file.json").await.unwrap();
    Ok(ApiResponse::JsonData(activities))
}
