use rocket::response::status::Created;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::Sender;
use rocket::tokio::time::{self, Duration};
use rocket::{Shutdown, State};

use crate::schema::*;
use crate::PgConnection;
use diesel::prelude::*;

use log::{debug, error, info};

use crate::models::{ApiError, NewReading, Reading};

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/events")]
pub(crate) async fn events(
    conn: PgConnection,
    queue: &State<Sender<Reading>>,
    mut end: Shutdown,
) -> EventStream![] {
    let _rx = queue.subscribe();
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            select! {
                _ = interval.tick() => {
                    match get_latest_reading(&conn).await {
                        Ok(reading) => {
                            yield Event::json(&reading);
                            yield Event::data(format!("{}°C", reading.temperature)).event("temperature");
                            yield Event::data(format!("{}%", reading.humidity)).event("humidity");
                            yield Event::data(format!("{} mbars", reading.pressure)).event("pressure");
                            yield Event::data(reading.air_purity.to_string()).event("air_purity");
                            yield Event::data(format!("{} pcs/ltr", reading.dust_concentration)).event("dust_concentration");
                        }
                        Err(e) => { error!("Err: failed to retrieve latest reading: {:?}", e) }
                    }
                }

                // Handle graceful shutdown of infinite EventStream
                _ = &mut end => {
                    info!("EventStream graceful shutdown requested, handling...");
                    break;
                }
            }
        }
    }
}

async fn get_latest_reading(conn: &PgConnection) -> Result<Reading, Json<ApiError>> {
    // Get the last inserted temperature value
    let reading = conn
        .run(move |c| {
            readings::table
                .order(readings::id.desc())
                .first::<Reading>(c)
        })
        .await
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        });

    debug!("Reading: {:?}", reading);

    reading
}

#[post("/readings/add", data = "<reading>")]
pub(crate) async fn create_reading(
    conn: PgConnection,
    reading: Json<NewReading>,
) -> Result<Created<Json<Reading>>, Json<ApiError>> {
    conn.run(move |c| {
        diesel::insert_into(readings::table)
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
