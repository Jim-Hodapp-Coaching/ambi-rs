use rocket::{State, Shutdown};
use rocket::response::status::Created;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::json::Json;
use rocket::tokio::sync::broadcast::Sender;
use rocket::tokio::time::{self, Duration};

use crate::schema::*;
use crate::PgConnection;
use diesel::prelude::*;

use crate::models::{Reading, NewReading, ApiError};

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/events")]
pub(crate) async fn events(conn: PgConnection, queue: &State<Sender<Reading>>, mut _end: Shutdown) -> EventStream![] {
    let _rx = queue.subscribe();
    std::println!("events()");
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            match get_latest_reading(&conn).await {
                Ok(reading) => {
                    yield Event::json(&reading);
                }
                Err(e) => { println!("Err: failed to retrieve latest reading: {:?}", e) }
            }

            interval.tick().await;
            // let msg = select! {
            //     msg = rx.recv() => match msg {
            //         Ok(msg) => msg,
            //         Err(RecvError::Closed) => break,
            //         Err(RecvError::Lagged(_)) => continue,
            //     },
            //     _ = &mut end => break,
            // };

            // std::println!("Subscribed remote client");
            // yield Event::json(&msg);
        }
    }
}

async fn get_latest_reading(conn: &PgConnection) -> Result<Reading, Json<ApiError>> {
    // Get the last inserted temperature value
    let reading = conn.run(move |c| {
        readings::table
            .order(readings::id.desc())
            .first::<Reading>(c)
    }).await
    .map(|a| a)
    .map_err(|e| {
        Json(ApiError {
            details: e.to_string(),
        })
    });

    println!("Reading: {:?}", reading);

    reading
}

#[post("/readings/add", data="<reading>")]
pub(crate) async fn create_reading(conn: PgConnection, reading: Json<NewReading>) -> Result<Created<Json<Reading>>, Json<ApiError>> {
    conn
    .run(move |c| {
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