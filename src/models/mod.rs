// models/mod.rs
use super::schema::config;
use super::schema::counter;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = config)]
pub struct Config {
    id: i32,
    text_string: String,
    user_role: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = counter)]
pub struct Counter {
    id: i32,
    count: i32,
}