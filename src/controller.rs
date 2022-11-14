use rocket::{State, Shutdown};
use rocket::response::status::Created;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::time::{self, Duration};
use rocket::tokio::select;

use diesel::result::Error;

use crate::models::{Reading, NewReading, ApiError};
use crate::schema::*;
//use crate::schema;
use crate::PgConnection;
use diesel::prelude::*;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub(crate) struct Message {
    pub temperature: f64,
}

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/events")]
pub(crate) async fn events(conn: PgConnection, queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    std::println!("events()");
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            match get_latest_temperature(&conn).await {
                Ok(t) => {
                    let msg = Message { temperature: t};
                    yield Event::json(&msg);
                }
                Err(e) => { println!("Err: failed to retrieve latest temperature") }
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

/// Receive a message from a form submission and broadcast it to any receivers.
// #[post("/temperature", data = "<form>")]
// pub(crate) fn get_temp(form: Form<Message>, queue: &State<Sender<Message>>) {
//     // A send 'fails' if there are no active subscribers. That's okay.
//     let _res = queue.send(form.into_inner());
// }

async fn get_latest_temperature(conn: &PgConnection) -> Result<f64, Error> {
    //use self::readings::dsl::*;

    // Get the last inserted temperature value
    let temperature = conn.run(move |c| {
        readings::table
            .select((readings::id, readings::temperature))
            //.filter(readings::id.eq(id.ok().unwrap()))
            .order(readings::id.desc())
            .first::<(i32, f64)>(c)
    }).await;

    Ok(temperature.ok().unwrap().1)
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