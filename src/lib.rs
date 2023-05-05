mod controller;
pub mod models;
pub mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::fs::{relative, FileServer};
use rocket::tokio::sync::broadcast::channel;
use rocket::Build;
use rocket_sync_db_pools::database;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use crate::controller::{create_reading, events};
use crate::models::Reading;

#[database("ambi_rs_dev")]
pub struct PgConnection(diesel::PgConnection);

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
pub fn rocket_builder() -> rocket::Rocket<Build> {
    rocket::build()
        .attach(PgConnection::fairing())
        .attach(CORS)
        .manage(channel::<Reading>(1024).0)
        .mount("/", routes![events])
        .mount("/api", routes![create_reading])
        .mount("/", FileServer::from(relative!("static")))
}
