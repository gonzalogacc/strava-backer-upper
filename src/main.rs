mod strava_endpoints;
mod settings;

mod strava;

mod schema;

use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    let app = Router::new().merge(strava_endpoints::strava_router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3007")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
