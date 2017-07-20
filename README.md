# Rust Rocket Api

#### A simple API built on Rust using Rocket and Diesel.
======================================================

## Installation

1. Install Rust using [Rustup](https://www.rustup.rs/)
2. [Rocket](https://rocket.rs/), the server framework, requires Rust Nightly which is compatible with Diesel as well.
- Once Rust is installed, I believe the stable version is installed by default, you can set it to Nightly with `rustup override set nightly`.
- If things stop working, ensure everything is using the latest version possible with `rustup update && cargo update`.
3. `git clone git@github.com:juliusdelta/imdb-rocket.git && cd imdb-rocket`
4. Create a `.env` file to connect to postgreSQL database.
- `echo DATABASE_URL=postgres://username:password@localhost/imdb-rocket > .env`
5. Run `cargo run` to start the server and everything should be good.
6. Enjoy the project!

## Contributions

## TroubleShooting
