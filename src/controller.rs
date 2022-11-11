use rocket::{
    response::status::Created,
    serde::json::Json,
};
use crate::models::{Reading, NewReading, ApiError};
use crate::schema;
use crate::PgConnection;
use diesel::prelude::*;

#[post("/readings/add", data="<reading>")]
pub async fn create_reading(conn: PgConnection, reading: Json<NewReading>) -> Result<Created<Json<Reading>>, Json<ApiError>> {
    conn
    .run(move |c| {
        diesel::insert_into(schema::readings::table)
            .values(&reading.into_inner())
            .get_result(c)
    })
    .await
    .map(|a| Created::new("/").body(Json(a)))
    .map_err(|e| {
        Json(ApiError {
            details: e.to_string(),
        })
    })
}