mod settings;
mod ingester;
mod endpoints;
mod strava_client;
use crate::strava_client::StravaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    let mut settings = settings::Settings::new();
    settings.load("MYNAME").expect("Could not load variable");
    settings.load("STRAVA_KEY").expect("Could not load variable");
    let value = settings.get_value("MYNAME").unwrap();

    let sc = StravaClient::init("");
    let athlete = sc.get_user().await.unwrap();
    println!("{:?}", athlete);
    Ok(())
}

