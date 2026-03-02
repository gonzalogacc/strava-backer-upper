mod strava_endpoints;
mod settings;

mod strava;

mod schema;
mod db_connection;

mod models;

use crate::db_connection::establish_connection;

use dotenv::dotenv;
use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); //TODO: Replace by the settings thingy

    let db_conn = establish_connection();

    // region: --- APP
    // build our application with a single route
    let app = Router::new()
        .merge(strava_endpoints::strava_router(db_conn.clone()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3007")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    // endregion: --- APP

    Ok(())
}

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::models::athlete::{create_athlete, AthleteRow, NewAthleteRow};
use crate::schema::athletes::dsl::athletes;

pub enum ApiResponse<T> {
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

pub struct ApiError {
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.message)).into_response()
    }
}