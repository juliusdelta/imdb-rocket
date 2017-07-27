#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;
mod db;
mod movie;
mod models;
mod error;

use db::DB;
use movie::{get_movies};
use models::*;
use rocket::JSON;
use rocket::response::status::{Created, NoContent};
use error::Error as ApiError;

#[get("/movies", format = "application/json")]
fn movies_get(db: DB) -> Result<JSON<Vec<Movie>, ApiError>> {
    let movies = Json(json!({get_movies(db.conn())}))?;
}

fn main() {
    rocket::ignite()
        .mount("/", routes![movies_get])
        .launch();
}
