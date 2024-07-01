use std::sync::Arc;
use rocket::{fairing::{Fairing, Info, Kind}, Data, Request, State};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::async_trait;


use crate::service::user::UserService;
use crate::types::ErrorResponse;



pub struct AuthorizeClientGuard;
#[async_trait]
impl<'r> FromRequest<'r> for AuthorizeClientGuard {
    type Error = ErrorResponse;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
       
        let user_service = match request.guard::<&State<Arc<UserService>>>().await {
            Outcome::Success(user_service) => user_service,
            Outcome::Error(_) | Outcome::Forward(_) => {
                return Outcome::Error((Status::InternalServerError, ErrorResponse{error: "internal_server_error".into()}));
            }
        };

        // check if request is for ws connection, this should be handled differently.
        // this can't use auth through headers, so instead uses query value.
        // allow access directly to handler, takes care of it..
        if let Some(route) = request.route() {
            if route.uri.path().to_string() == "/api/collaboration/case/<case_id>/connect/<user_id>" {
                return Outcome::Success(AuthorizeClientGuard);
            }
        }

        // if it is a common request, look for the token in the header
        match request.headers().get_one("Authorization") {
            Some(header) => {
                if header.starts_with("Bearer ") {
                    let token = &header[7..];
                    match user_service.check(token.to_string()).await {
                        Ok(_) => {
                            Outcome::Success(AuthorizeClientGuard)
                        },
                        Err(e) => {
                            eprintln!("invalid token: {}", e);
                            Outcome::Error((Status::Unauthorized, ErrorResponse {
                                error: "invalid token".into() 
                            }))
                        },
                    }
                } else {
                    Outcome::Error((Status::Unauthorized, ErrorResponse {
                        error: "invalid token format".into() 
                    }))
                }
            },
            None => Outcome::Error((Status::BadRequest, ErrorResponse {
                error: "missing authorization header".into() 
            }))
        }
    }
}




pub struct Logger;
#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        info!("Received request: {} {}", request.method(), request.uri());
    }
}