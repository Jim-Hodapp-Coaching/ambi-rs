//use async_stream::stream;
use axum::{
    extract::State,
    extract::TypedHeader,
    http::StatusCode,
    response::sse::{Event, Sse},
    response::Json,
};
use futures::stream::{self, Stream};
//use futures_util::select;
use diesel::prelude::*;
use std::{convert::Infallible, time::Duration};
use tokio::runtime;
use tokio_stream::StreamExt as _;

use crate::schema::*;

use crate::models::{ApiError, NewReading, Reading};

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
// #[get("/events")]
// pub(crate) async fn events(
//     conn: PgConnection,
//     queue: &State<Sender<Reading>>,
//     mut end: Shutdown,
// ) -> EventStream![] {
//     let _rx = queue.subscribe();
//     EventStream! {
//         let mut interval = time::interval(Duration::from_secs(5));
//         loop {
//             select! {
//                 _ = interval.tick() => {
//                     match get_latest_reading(&conn).await {
//                         Ok(reading) => {
//                             yield Event::json(&reading);
//                             yield Event::data(format!("{}°C", reading.temperature)).event("temperature");
//                             yield Event::data(format!("{}%", reading.humidity)).event("humidity");
//                             yield Event::data(format!("{} mbars", reading.pressure)).event("pressure");
//                             yield Event::data(format!("{}", reading.air_purity)).event("air_purity");
//                             yield Event::data(format!("{} pcs/ltr", reading.dust_concentration)).event("dust_concentration");
//                         }
//                         Err(e) => { error!("Err: failed to retrieve latest reading: {:?}", e) }
//                     }
//                 }

//                 // Handle graceful shutdown of infinite EventStream
//                 _ = &mut end => {
//                     info!("EventStream graceful shutdown requested, handling...");
//                     break;
//                 }
//             }
//         }
//     }
// }

pub(crate) async fn events_handler(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    //let mut interval = tokio::time::interval(Duration::from_secs(5));

    // let astream = stream! {
    // match get_latest_reading(&conn).await {
    //     Ok(reading) => {
    //         yield Event::json(&reading);
    //         yield Event::data(format!("{}°C", reading.temperature)).event("temperature");
    //         yield Event::data(format!("{}%", reading.humidity)).event("humidity");
    //         yield Event::data(format!("{} mbars", reading.pressure)).event("pressure");
    //         yield Event::data(format!("{}", reading.air_purity)).event("air_purity");
    //         yield Event::data(format!("{} pcs/ltr", reading.dust_concentration)).event("dust_concentration");
    //     }
    //     Err(e) => { error!("Err: failed to retrieve latest reading: {:?}", e) }
    // }
    // };

    // A `Stream` that repeats an event every 5 seconds
    // let stream = stream::repeat_with(|| async {
    //         let event = match get_latest_reading(axum::extract::State(pool)).await {
    //             //Ok(reading) => { Event::data(format!("{}°C", reading.temperature)).event("temperature") }
    //             Ok(reading) => { Event::default().data("Successfully retrieved latest reading") }
    //             Err(e) => { Event::default().data("Failed to retrieve latest reading") }
    //         }
    //     })
    //     .map(Ok)
    //     .throttle(Duration::from_secs(5));

    // match get_latest_reading(axum::extract::State(pool)).await {
    //     Ok(reading) => {
    //         println!("{}°C", reading.temperature);
    //     }
    //     Err(e) => { println!("Err: failed to retrieve latest reading: {:?}", e) }
    // }

    // TODO: maybe we pass in a stream of Event instances filled with Readings into event_handler instead of
    // asking this method to retrieve Readings from the DB
    let stream =
        stream::repeat_with(
            || match get_latest_reading(axum::extract::State(pool)).await {
                Ok(reading) => Event::default()
                    .data(format!("{}°C", reading.temperature))
                    .event("temperature"),
                Err(e) => Event::default()
                    .data("Err: failed to retrieve latest reading: {e}")
                    .event("error"),
            },
        )
        .map(Ok)
        .throttle(Duration::from_secs(5));

    // let stream = stream::repeat_with(|| Event::default().data("hi!"))
    //     .map(Ok)
    //     .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keep-alive-text"),
    )
}

// async fn get_latest_reading(conn: &PgConnection) -> Result<Reading, Json<ApiError>> {
//     // Get the last inserted temperature value
//     let reading = conn
//         .run(move |c| {
//             readings::table
//                 .order(readings::id.desc())
//                 .first::<Reading>(c)
//         })
//         .await
//         .map_err(|e| {
//             Json(ApiError {
//                 details: e.to_string(),
//             })
//         });

//     debug!("Reading: {:?}", reading);

//     reading
// }

// #[post("/readings/add", data = "<reading>")]
// pub(crate) async fn create_reading(
//     conn: PgConnection,
//     reading: Json<NewReading>,
// ) -> Result<Created<Json<Reading>>, Json<ApiError>> {
//     conn.run(move |c| {
//         diesel::insert_into(readings::table)
//             .values(&reading.into_inner())
//             .get_result(c)
//     })
//     .await
//     .map(|a| Created::new("/").body(Json(a)))
//     .map_err(|e| {
//         Json(ApiError {
//             details: e.to_string(),
//         })
//     })
// }

pub(crate) async fn create_reading(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_reading): Json<NewReading>,
) -> Result<Json<Reading>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(readings::table)
                .values(new_reading)
                .returning(Reading::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub(crate) async fn get_latest_reading(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Reading>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        //.interact(|conn| users::table.select(Reading::as_select()).load(conn))
        .interact(|conn| {
            readings::table
                .order(readings::id.desc())
                .first::<Reading>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
