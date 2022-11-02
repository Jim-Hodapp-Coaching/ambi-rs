use serde::Deserialize;
use diesel::Insertable;
use crate::schema::readings;

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "readings"]
pub struct Reading {
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub dust_concentration: Option<f64>,
    pub pressure: Option<i32>,
    pub air_purity: Option<String>,
}