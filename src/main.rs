#[derive(Debug)]
struct Movie {
    title: String,
    rating: String,
    director: String,
    score: i32
}

impl Movie {
    fn new () -> Movie {
        Movie {
            title: String::from("The Matrix"),
            rating: String::from("R"),
            director: String::from("The Wachowskis"),
            score: 87
        }
    }
}

fn main() {
    let movie = Movie::new();
    println!("{:?}", movie);
}
