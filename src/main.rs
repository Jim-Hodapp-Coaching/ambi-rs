#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::resource("/readings/add").route(web::post().to(add_reading))),
        )
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

async fn add_reading(reading: web::Json<models::Reading>) -> Result<String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let new_reading = models::Reading {
        temperature: reading.temperature,
        humidity: reading.humidity,
        dust_concentration: reading.dust_concentration,
        pressure: reading.pressure,
        air_purity: reading.air_purity.to_owned(),
    };

    diesel::insert_into(schema::readings::table)
        .values(&new_reading)
        .execute(&mut connection)
        .unwrap();

    Ok("Created reading".to_owned())
}
