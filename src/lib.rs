// mod controller;
// pub mod models;
// pub mod schema;

// use axum::{
//     extract::State,
//     http::StatusCode,
//     response::Json,
//     routing::{get, post},
//     Router,
// };
// use diesel::prelude::*;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// use std::net::SocketAddr;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use crate::controller::{create_reading, events};
// use crate::models::Reading;

// // This embeds the migrations into the application binary
// // the migration path is relative to the `CARGO_MANIFEST_DIR`
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

// #[database("ambi_rs_dev")]
// pub struct PgConnection(diesel::PgConnection);

// pub struct CORS;

// #[rocket::async_trait]
// impl Fairing for CORS {
//     fn info(&self) -> Info {
//         Info {
//             name: "Add CORS headers to responses",
//             kind: Kind::Response,
//         }
//     }

//     async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
//         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
//         response.set_header(Header::new(
//             "Access-Control-Allow-Methods",
//             "POST, GET, PATCH, OPTIONS",
//         ));
//         response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
//     }
// }

// #[launch]
// pub fn rocket_builder() -> rocket::Rocket<Build> {
//     rocket::build()
//         .attach(PgConnection::fairing())
//         .attach(CORS)
//         .manage(channel::<Reading>(1024).0)
//         .mount("/", routes![events])
//         .mount("/api", routes![create_reading])
//         .mount("/", FileServer::from(relative!("static")))
// }
