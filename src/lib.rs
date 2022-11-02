mod routes;
pub mod db;
pub mod models;
pub mod schema;

#[macro_use] 
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::Build;
use rocket_sync_db_pools::database;
use crate::routes::create_reading;

#[database("ambi_rs_dev")]
pub struct PgConnection(diesel::PgConnection);

#[launch]
pub fn rocket_builder() -> rocket::Rocket<Build> {
    rocket::build()
    .attach(PgConnection::fairing())
    .mount("/", routes![create_reading])
}