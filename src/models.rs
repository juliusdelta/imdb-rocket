extern crate rocket;

use super::schema::movies;
use self::rocket::{Request, Data};
use self::rocket::data::{self, FromData};
use self::rocket::http::Status;
use self::rocket::Outcome::*;
use serde_json;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Movie {
    id: i32,
    title: String,
    rating: String,
    director: String,
    score: i32
}
