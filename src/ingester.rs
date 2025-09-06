use chrono::{DateTime, Datelike, Utc};
use serde;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct Athlete {
    pub id: i64,

    #[serde(default)]
    pub username: Option<String>,
    pub firstname: String,
    lastname: String,
    profile: Url,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Athlete {
    fn new(athlete_data: &str) -> Result<Athlete, &'static str> {
        let at: Athlete = serde_json::from_str(athlete_data).unwrap();
        Ok(at)
    }
}

#[derive(Deserialize)]
struct ActivityAthlete {
    id: i64,
}

#[derive(Deserialize)]
struct Activity {
    id: i64,
    athlete: ActivityAthlete,
    name: String,
    distance: i32,
    moving_time: i32,
    elapsed_time: i32,
    start_date: DateTime<Utc>,
}

impl Activity {
    fn new(data: &str) -> Result<Activity, serde_json::Error> {
        serde_json::from_str(data)
    }
}

#[derive(Deserialize)]
struct ActivityStream {
    #[serde(alias = "type")]
    stream_type: String,
    data: Vec<f32>,
    series_type: String,
    original_size: i32,
    resolution: String,
}

impl ActivityStream {
    fn from(stream_data: &str) -> ActivityStream {
        let stream = serde_json::from_str(stream_data).unwrap();
        stream
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
struct ActivityStreams {
    streams: Vec<ActivityStream>,
}

impl ActivityStreams {
    fn from(streams_data: &str) -> ActivityStreams {
        let acts = serde_json::from_str(streams_data).unwrap();
        acts
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_athlete() {
        let input = r#"{"id":28853829,"username":null,"resource_state":2,"firstname":"Gonzalo","lastname":"Garcia","bio":"","city":"","state":"","country":"","sex":null,"premium":false,"summit":false,"created_at":"2018-03-09T23:01:47Z","updated_at":"2024-01-28T21:00:13Z","badge_type_id":0,"weight":84.6,"profile_medium":"https://graph.facebook.com/10156169906188476/picture?height=256\u0026width=256","profile":"https://graph.facebook.com/10156169906188476/picture?height=256\u0026width=256","friend":null,"follower":null}"#;
        let atlh = Athlete::new(input).unwrap();

        assert_eq!(atlh.id, 28853829);
        assert_eq!(atlh.username.unwrap_or("".to_string()), "");

        let input = r#"{"id":28853829,"username": "gonza","resource_state":2,"firstname":"Gonzalo","lastname":"Garcia","bio":"","city":"","state":"","country":"","sex":null,"premium":false,"summit":false,"created_at":"2018-03-09T23:01:47Z","updated_at":"2024-01-28T21:00:13Z","badge_type_id":0,"weight":84.6,"profile_medium":"https://graph.facebook.com/10156169906188476/picture?height=256\u0026width=256","profile":"https://graph.facebook.com/10156169906188476/picture?height=256\u0026width=256","friend":null,"follower":null}"#;
        let athl = Athlete::new(input).unwrap();

        assert_eq!(athl.id, 28853829);
        assert_eq!(
            athl.username.unwrap_or("can't find username".to_string()),
            "gonza"
        );
        assert_eq!(athl.created_at.year(), 2018);
        assert_eq!(
            athl.profile.as_str(),
            "https://graph.facebook.com/10156169906188476/picture?height=256&width=256"
        );
    }
    #[test]
    fn test_parse_activity() {
        let act_data = r#"{"id" : 123456778928065, "resource_state" : 3, "external_id" : null, "upload_id" : null, "athlete" : {"id" : 12343545645788, "resource_state" : 1}, "name" : "Chill Day", "distance" : 0, "moving_time" : 18373, "elapsed_time" : 18373, "total_elevation_gain" : 0, "type" : "Ride", "sport_type" : "MountainBikeRide", "start_date" : "2018-02-20T18:02:13Z", "start_date_local" : "2018-02-20T10:02:13Z", "timezone" : "(GMT-08:00) America/Los_Angeles", "utc_offset" : -28800, "achievement_count" : 0, "kudos_count" : 0, "comment_count" : 0, "athlete_count" : 1, "photo_count" : 0, "map" : {"id" : "a12345678908766", "polyline" : null, "resource_state" : 3}, "trainer" : false, "commute" : false, "manual" : true, "private" : false, "flagged" : false, "gear_id" : "b453542543", "from_accepted_tag" : null, "average_speed" : 0, "max_speed" : 0, "device_watts" : false, "has_heartrate" : false, "pr_count" : 0, "total_photo_count" : 0, "has_kudoed" : false, "workout_type" : null, "description" : null, "calories" : 0, "segment_efforts" : [ ]}"#;
        let act = Activity::new(act_data).unwrap();
        assert_eq!(act.id, 123456778928065);
        assert_eq!(act.athlete.id, 12343545645788);
        assert_eq!(act.name, "Chill Day");
        assert_eq!(act.distance, 0);
        assert_eq!(act.moving_time, 18373);
        assert_eq!(act.elapsed_time, 18373);
        assert_eq!(act.start_date.day(), 20)
    }

    #[test]
    fn test_activity_stream() {
        let stream_data = r#"[ {"type" : "distance", "data" : [ 2.9, 5.8, 8.5, 11.7, 15, 19, 23.2, 28, 32.8, 38.1, 43.8, 49.5 ], "series_type" : "distance", "original_size" : 12, "resolution" : "high"}]"#;
        let stream = ActivityStreams::from(stream_data);
        assert_eq!(stream.streams[0].stream_type, "distance");
    }
}
