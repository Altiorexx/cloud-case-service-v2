use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;


use crate::database::core::{CoreDatabase, new_core_database};
use crate::types::core_database::{User, Service};
use crate::types::ErrorResponse;
use super::middleware_handler;


pub struct UserHandler {
    core: CoreDatabase
}

impl UserHandler {
    pub async fn new() -> UserHandler {
        UserHandler {
            core: new_core_database().await
        }
    }
}



#[derive(Debug, serde::Deserialize)]
pub struct CreateUserBody {
    pub name: String,
    pub email: String,
    pub password: String,
    pub last_login: String,
}

#[post("/api/user/create", data = "<data>")]
pub async fn create_user(_guard: middleware_handler::AuthorizeClientGuard, handler: &State<UserHandler>, data: Json<CreateUserBody>) -> String {
    match handler.core.create_user(&data.name, &data.email, &data.password, &data.last_login, false).await {
        Ok(_) => format!("successfully created user").to_string(),
        Err(e) => format!("error creating user: {}", e).to_string()
    } 
}

#[get("/<user_id>")]
pub async fn get_user_by_id(_guard: middleware_handler::AuthorizeClientGuard, handler: &State<UserHandler>, user_id: &str) -> Result<Json<User>, String> {
    match handler.core.read_user_by_id(user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.to_string())
    }
}

#[get("/services")]
pub async fn get_services(handler: &State<UserHandler>) -> Result<Custom<Json<Vec<Service>>>,Custom<Json<ErrorResponse>>> {
    match handler.core.read_services().await {
        Ok(services) => Ok(Custom(Status::Ok, Json(services))),
        Err(e) => {
            println!("error reading services: {}", e);
            Err(Custom(Status::InternalServerError, Json(ErrorResponse{error: "error reading services".into()})))
        }
    }
}




