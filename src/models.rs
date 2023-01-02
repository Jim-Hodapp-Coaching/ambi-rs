use serde::{ Serialize, Deserialize};
use diesel::Insertable;
use crate::schema::readings;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Reading {
    pub id: i32,
    pub temperature: f64,
    pub humidity: f64,
    pub dust_concentration: f64,
    pub pressure: i32,
    pub air_purity: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "readings"]
pub struct NewReading {
    pub temperature: f64,
    pub humidity: f64,
    pub dust_concentration: f64,
    pub pressure: i32,
    pub air_purity: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}