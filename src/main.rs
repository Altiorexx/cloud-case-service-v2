#[macro_use] extern crate rocket;
pub mod config;
pub mod types;
pub mod api;
pub mod database;
pub mod service;

use std::env;
use rocket::Config;

use api::case_handler;
use api::user_handler;
use api::collaboration_handler::{self, new_collaboration_handler};
use api::middleware_handler::Logger;
use api::cors::{CORS, all_options};
use service::user::UserService;


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::load_environment_var_file();
    
     // get the port from the environment variable, panicking if not set or invalid
    let port: u16 = env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse()
        .expect("PORT must be a valid number");

    // create a custom configuration
    let figment = Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    rocket::build()
    .configure(figment)
    .manage(case_handler::CaseHandler::new().await)
    .manage(user_handler::UserHandler::new().await)
    .manage(new_collaboration_handler().await)
    .manage(UserService::new())
    .attach(CORS)
    .attach(Logger)
    .mount("/", routes![

        collaboration_handler::connect,

        case_handler::create_cis18_case,
        case_handler::rename_case,
        case_handler::delete_case,

        case_handler::get_case,
        case_handler::get_cases,

        user_handler::get_services,
        user_handler::get_user_by_id,
        user_handler::create_user,

        all_options
    ])
    .launch().await?;
    Ok(())
}


