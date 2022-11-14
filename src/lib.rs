mod controller;
pub mod models;
pub mod schema;

#[macro_use] 
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::Build;
use rocket::fs::{relative, FileServer};
use rocket_sync_db_pools::database;
use rocket::tokio::sync::broadcast::channel;

use crate::controller::{create_reading, events, Message};

#[database("ambi_rs_dev")]
pub struct PgConnection(diesel::PgConnection);

#[launch]
pub fn rocket_builder() -> rocket::Rocket<Build> {
    rocket::build()
    .attach(PgConnection::fairing())
    .manage(channel::<Message>(1024).0)
    .mount("/", routes![events])
    .mount("/api", routes![create_reading])
    .mount("/", FileServer::from(relative!("static")))
}