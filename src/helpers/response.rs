use serde::Serialize;
use serde_json::Value;
use actix_web::{HttpResponse, http::StatusCode};

#[derive(Serialize, Debug)]
pub struct ResponseStruct {
    status: String,
    message: String,
    data: Vec<Value>,
}

#[doc = "Create a response template"]
pub fn response_json(status: String, message: String, data: Vec<Value>) -> HttpResponse {
    // init response
    let mut code = StatusCode::ACCEPTED;
    let stats = status.to_owned().to_lowercase();

    // set status code
    if stats == "success" {
        code = StatusCode::OK;
    } else if stats == "error" {
        code = StatusCode::INTERNAL_SERVER_ERROR;
    } else if stats == "failed" {
        code = StatusCode::BAD_REQUEST;
    } else if stats == "unauthorize" {
        code = StatusCode::UNAUTHORIZED;
    } else if stats == "forbidden" {
        code = StatusCode::FORBIDDEN;
    }

    // set response body
    let body = ResponseStruct {
        status: status.to_string(),
        message: message.to_string(),
        data,
    };

    // return response
    HttpResponse::build(code)
        .content_type("application/json")
        .json(body)
}
