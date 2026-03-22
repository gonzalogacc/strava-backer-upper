use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use crate::{schema::athletes::dsl::athletes, strava::parsers::Athlete};
use axum::http::StatusCode;
use deadpool_diesel::postgres::{Object, Pool}; // Import the Pool type
use crate::{ApiError, ApiResponse};

#[derive(Insertable)]
#[diesel(table_name=crate::schema::athletes)]
pub struct NewAthleteRow {
    pub id: i64,
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::athletes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AthleteRow {
    pub id: i64,
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub another_column: Option<String>,
}

pub async fn create_athlete(
    conn: &Object,
    athlete: &Athlete
) -> Result<(), ApiError> {
    use crate::schema::athletes;
    use crate::schema::athletes::dsl::*;
    println!("heheheh");
    let new_athlete = NewAthleteRow {
        id: athlete.id,
        username: athlete.username.clone(),
        firstname: Some(athlete.firstname.clone()),
        lastname: Some(athlete.lastname.clone()),
        created_at: Utc::now().naive_utc(),
        updated_at: Some(Utc::now().naive_utc()),
    };
    println!("Here!!!");
    conn.interact(move |conn| {
        diesel::insert_into(athletes)
            .values(&new_athlete)
            .on_conflict(id)
            .do_nothing()
            .execute(conn)
    })
    .await
    .map_err(|_| ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "DB interaction error".to_string() })?
    .map_err(|_| ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "DB execute error".to_string() })?;

    return Ok(())
}