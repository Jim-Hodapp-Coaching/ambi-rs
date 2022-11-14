use serde::{ Serialize, Deserialize};
use diesel::Insertable;
use crate::schema::readings;

#[derive(Debug, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Reading {
    pub id: i32,
    pub temperature: f64,
    pub humidity: Option<f64>,
    pub dust_concentration: Option<f64>,
    pub pressure: Option<i32>,
    pub air_purity: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "readings"]
pub struct NewReading {
    pub temperature: f64,
    pub humidity: Option<f64>,
    pub dust_concentration: Option<f64>,
    pub pressure: Option<i32>,
    pub air_purity: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}