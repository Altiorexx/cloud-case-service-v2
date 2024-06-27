use rocket::{fairing::{Fairing, Info, Kind}, Data, Request};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::outcome::Outcome;
use rocket::async_trait;


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

pub struct ApiKeyGuard;
#[async_trait]
impl<'r> FromRequest<'r> for ApiKeyGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        
        match request.headers().get_one("x-api-key") {
            Some(_key) => Outcome::Success(ApiKeyGuard),
            None => Outcome::Error((Status::BadRequest, ()))
        }

    }
}

