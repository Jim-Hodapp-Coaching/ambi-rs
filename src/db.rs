use crate::schema;
use crate::models::Reading;
use crate::PgConnection;
use diesel::prelude::*;

async fn add_reading(conn: PgConnection, reading: Reading) -> Result<String, String> {

    let new_reading = Reading {
        temperature: reading.temperature,
        humidity: reading.humidity,
        dust_concentration: reading.dust_concentration,
        pressure: reading.pressure,
        air_purity: reading.air_purity.to_owned(),
    };

    conn.run(move |c| {
        diesel::insert_into(schema::readings::table)
        .values(&new_reading)
        .get_result(c)
    });

    Ok("Created reading".to_owned())
}