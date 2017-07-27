use diesel::result::Error;
use diesel;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;
use schema::movies;

pub fn get_movies(conn: &PgConnection) -> Result<Vec<Movie>, Error> {
    movies::table
        .load::<Movie>(conn)
}
