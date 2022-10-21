use crate::schema::readings;
use diesel::Insertable;
use serde::Deserialize;

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = readings)]
pub struct Reading {
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub dust_concentration: Option<f64>,
    pub pressure: Option<i32>,
    pub air_purity: Option<String>,
}
