use crate::response::GenericResponse;
use actix_web::{route, HttpResponse, Responder};

#[route("", method = "GET", method = "POST", method = "PUT", method = "DELETE")]
async fn get_welcome() -> impl Responder {
    const MESSAGE: &str = "Welcome to the API";

    let response_json = &GenericResponse {
        status: "OK".to_string(),
        message: MESSAGE.to_string(),
        data: "".to_string(),
    };

    HttpResponse::Ok().json(response_json)
}