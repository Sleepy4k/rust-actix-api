use time::Duration;
use serde::Serialize;
use serde_json::Value;
use actix_web::{HttpResponse, http::StatusCode, cookie::{Cookie, SameSite}};

#[derive(Serialize, Debug)]
pub struct ResponseStruct {
  status: String,
  message: String,
  data: Vec<Value>,
}

#[doc = "Set status code for response"]
fn setup_code(status: String) -> StatusCode {
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
  } else if stats == "not found" {
    code = StatusCode::NOT_FOUND;
  }

  code
}

#[doc = "Create a response template"]
pub fn response_json(status: String, message: String, data: Vec<Value>) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

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

#[doc = "Create a response template with cookie"]
pub fn response_json_with_cookie(status: String, message: String, data: Vec<Value>, cookie_type: String, cookie_title: String, cookie_value: String) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // set response body
  let body = ResponseStruct {
    status: status.to_string(),
    message: message.to_string(),
    data,
  };

  // init response
  let mut response = HttpResponse::build(code);

  // init cookie
  let method = cookie_type.to_owned().to_lowercase();

  if method == "set" {
    let cookie = Cookie::build(cookie_title, cookie_value)
      .secure(true)
      .http_only(false)
      .same_site(SameSite::Strict)
      .max_age(Duration::days(7))
      .finish();

    response.cookie(cookie);
  } else if method == "remove" {
    let cookie = Cookie::build(cookie_title, cookie_value)
      .secure(true)
      .http_only(false)
      .same_site(SameSite::Strict)
      .max_age(Duration::seconds(0))
      .finish();

    response.cookie(cookie);
  }

  // return response
  response
    .content_type("application/json")
    .json(body)
}