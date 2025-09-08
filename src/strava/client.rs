use std::collections::HashSet;
use crate::strava::parsers::{Athlete, Activity};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct LoginUrl {
    pub url: String,
}

// request to exchange the code from strava
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenSet {
    expires_at: i64,
    expires_in: i64,
    token_type: String,
    refresh_token: String,
    access_token: String,
}

pub struct StravaClient {
    base_url: String,
    client_id: i32,
    client_secret: String,
    token_file: String,
}

impl StravaClient {
    pub fn init(base_url: &str, client_secret: &str) -> StravaClient {
        StravaClient {
            base_url: base_url.to_string(),
            client_id: 118327,
            client_secret: client_secret.to_string(),
            token_file: "./tokens.txt".to_string(),
        }
    }

    pub async fn login_link(&self) -> LoginUrl {
        let mut url_builder = Url::parse("https://www.strava.com").unwrap();
        url_builder.set_path("/oauth/authorize");
        url_builder
            .query_pairs_mut()
            .append_pair("client_id", &self.client_id.to_string())
            .append_pair("redirect_uri", "http://localhost:3007/token_exchange")
            .append_pair("response_type", "code")
            .append_pair("scope", "activity:read_all")
            .append_pair("state", "123456");

        LoginUrl {
            url: url_builder.to_string(),
        }
    }

    pub async fn code_exchange(&self, code: &str) -> Result<TokenSet, reqwest::Error> {
        let mut exchange_url = Url::parse(&self.base_url).unwrap();
        exchange_url.set_path("/api/v3/oauth/token");
        exchange_url
            .query_pairs_mut()
            .append_pair("client_id", &self.client_id.to_string())
            .append_pair("client_secret", &self.client_secret)
            .append_pair("code", code)
            .append_pair("grant_type", "authorization_code");

        let client = reqwest::Client::new();
        let response = client.post(exchange_url.to_string()).send().await?;
        let token_set = response.error_for_status()?.json::<TokenSet>().await?;

        // Save this to a file before coming back to the function, read it just to be sure
        self.write_to_file(&token_set, &self.token_file)
            .expect("Failed writing tokes to file");

        Ok(token_set)
    }

    pub async fn refresh_token(&self) -> Result<TokenSet, reqwest::Error> {
        let content = self
            .read_from_file(&self.token_file)
            .expect("Could not read file");

        let mut refresh_url = Url::parse(&self.base_url).unwrap();
        refresh_url.set_path("/api/v3/oauth/token");
        refresh_url
            .query_pairs_mut()
            .append_pair("client_id", &self.client_id.to_string())
            .append_pair("client_secret", &self.client_secret)
            .append_pair("grant_type", "refresh_token")
            .append_pair("refresh_token", &content.refresh_token);

        let client = reqwest::Client::new();
        let response = client.post(refresh_url.to_string()).send().await?;
        let token_set = response.error_for_status()?.json::<TokenSet>().await?;

        // Save this to a file before coming back to the function, read it just to be sure
        self.write_to_file(&token_set, &self.token_file)
            .expect("Failed writing tokes to file");

        Ok(token_set)
    }

    fn write_to_file(&self, token_set: &TokenSet, token_file: &str) -> std::io::Result<()> {
        let mut file = File::create(token_file)?;
        file.write_all(serde_json::to_string(token_set)?.as_bytes())?;
        Ok(())
    }

    fn read_from_file(&self, filename: &str) -> serde_json::error::Result<TokenSet> {
        let content = fs::read_to_string(filename).expect("Could not open token file");
        serde_json::from_str(&content)
    }

    pub async fn get_user(&self) -> Result<Athlete, reqwest::Error> {
        let content = self
            .read_from_file(&self.token_file)
            .expect("Could not read file");

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/api/v3/athlete", &self.base_url))
            .header(
                "Authorization",
                "Bearer ".to_string() + &content.access_token,
            )
            .send()
            .await?;
        let athlete = response.error_for_status()?.json::<Athlete>().await?;
        Ok(athlete)
    }

    pub async fn get_activities(&self) -> Result<Vec<Activity>, reqwest::Error> {
        let content = self
            .read_from_file(&self.token_file)
            .expect("Could not read file");

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/api/v3/activities", &self.base_url))
            .header(
                "Authorization",
                "Bearer ".to_string() + &content.access_token,
            )
            .send()
            .await?;
        let activity = response.error_for_status()?.json::<Vec<Activity>>().await?;
        Ok(activity)
    }

    pub async fn write_activities(&self, activities: &Vec<Activity>, activities_file: &str) -> std::io::Result<()> {
        let mut act_set = HashSet::new();

        let file = File::open(activities_file)?;
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let json = Activity::new(&line?);
            act_set.insert(json?.id);
        }

        let mut f = OpenOptions::new()
            .write(true)
            .open(activities_file)
            .unwrap();

        for act in activities {
            if !act_set.contains(&act.id) {
               f.write_all(serde_json::to_string(act)?.as_bytes()).expect("Could not write");
                let _ = f.write_all("\n".as_bytes());
            }
        }
        let _ = f.flush().unwrap();
        Ok(())
    }
}

#[tokio::test]
async fn test_get_user_request() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let body_string = r#"{"id":28853829,"username":null,"resource_state":2,"firstname":"Gonzalo","lastname":"Garcia","bio":"","city":"","state":"","country":"","sex":null,"premium":false,"summit":false,"created_at":"2018-03-09T23:01:47Z","updated_at":"2024-01-28T21:00:13Z","badge_type_id":0,"weight":84.6,"profile_medium":"https://graph.facebook.com/10156169906188476/picture?height=256\u0026width=256","profile":"https://graph.facebook.com/10156169906188476/picture?height=256\u0026width=256","friend":null,"follower":null}"#;

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/athlete"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body_string))
        .mount(&mock_server)
        .await;

    let sc = StravaClient::init(&mock_server.uri(), "mock-app-token");
    let at = sc.get_user().await.unwrap();
    assert_eq!(at.id, 28853829);
}
