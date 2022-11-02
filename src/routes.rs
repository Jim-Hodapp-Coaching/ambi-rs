use crate::db;

use rocket::response::status;
use rocket::serde::json::Json;
use crate::models::Reading;

#[post("/readings", data="<reading>")]
pub fn create_reading(reading: Json<Reading>) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", 10)))
}