use crate::ingester::Athlete;
use std::fmt;
use crate::settings;

pub struct StravaClient {
    api_token: String,
}

impl StravaClient {
    pub fn init(strava_token: &str) -> StravaClient {
        StravaClient {
            api_token: strava_token.to_string(),
        }
    }

    pub async fn get_user(&self) -> Result<Athlete, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.strava.com/api/v3/athlete")
            .header(
                "Authorization",
                "Bearer ".to_string() + &self.api_token.to_string(),
            )
            .send()
            .await?;
        let athlete = response.error_for_status()?.json::<Athlete>().await?;
        Ok(athlete)
    }
}

#[tokio::test]
async fn test_get_user_request() {
    dotenv::dotenv().ok();
    let mut settings = settings::Settings::new();
    settings.load("STRAVA_KEY").expect("Could not load variable");

    let sc = StravaClient::init(settings.get_value("STRAVA_KEY").expect("Could not get strava key from env"));
    let at = sc.get_user().await.unwrap();
    assert_eq!(at.id, 28853829);
}
