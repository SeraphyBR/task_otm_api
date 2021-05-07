#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod controllers;
mod models;

use rocket_cors::CorsOptions;

use crate::controllers::problem;

fn get_rocket_instance() -> rocket::Rocket {
    let cors = CorsOptions::default().to_cors().unwrap();

    let problem_routes = routes![
        problem::solve
    ];

    rocket::ignite()
        .mount("/problem", problem_routes)
        .attach(cors)
}

fn main() {
    let rocket = get_rocket_instance();
    rocket.launch();
}
