use crate::schema::readings;
use diesel::Insertable;
use serde::Deserialize;

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "readings"]
pub struct Reading {
    pub id: i32,
    pub temperature: f64,
    pub humidity: f64,
    pub dust_concentration: f64,
    pub pressure: i32,
    pub air_purity: String,
}
