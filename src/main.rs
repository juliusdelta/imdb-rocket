#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;

extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Movie {
    id: i32,
    title: String,
    rating: String,
    director: String,
    score: i32
}

impl Movie {
    fn new (id: i32, title: String, rating: String, director: String, score: i32) -> Movie {
        Movie {
            id: id,
            title: String::from(title),
            rating: String::from(rating),
            director: String::from(director),
            score: score
        }
    }
}

#[get("/movies")]
fn movie() -> Json<Movie> {
    let title = String::from("The Matrix");
    let rating = String::from("R");
    let director = String::from("The Wachowski's");

    Json(Movie::new(1, title, rating, director, 87))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![movie])
        .launch();
}
