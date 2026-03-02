use diesel::pg::Pg;
use diesel::prelude::*;
use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use diesel::prelude::*;

use deadpool_diesel::postgres::Manager;
use deadpool_diesel::postgres::Pool;

pub fn establish_connection() -> Pool {
    dotenv().ok(); //TODO: Replace by the settings thingy

    let database_url = env::var("DATABASE_URL").expect("Database_url not set");
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(10)
        .build()
        .unwrap();
    return pool
}
