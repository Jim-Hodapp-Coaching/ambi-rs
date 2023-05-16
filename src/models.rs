use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::readings;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable)]
pub struct Reading {
    pub id: i32,
    pub temperature: f64,
    pub humidity: f64,
    pub dust_concentration: f64,
    pub pressure: i32,
    pub air_purity: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = readings)]
pub struct NewReading {
    pub temperature: f64,
    pub humidity: f64,
    pub dust_concentration: f64,
    pub pressure: i32,
    pub air_purity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub details: String,
}
