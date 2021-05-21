#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;

mod controllers;
mod models;

use rocket_cors::CorsOptions;
use std::error::Error;

use crate::controllers::problem;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsOptions::default().to_cors()?;

    let problem_routes = routes![
        problem::solve
    ];

    rocket::build()
        .mount("/problem", problem_routes)
        .attach(cors)
        .launch().await?;

    Ok(())
}
