#[derive(Queryable)]
pub struct Movie {
    id: i32,
    title: String,
    rating: String,
    director: String,
    score: i32
}
