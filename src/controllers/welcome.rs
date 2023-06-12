use std::env;
use actix_web::Responder;

use crate::helpers::response::*;

#[doc = "Welcome route"]
pub async fn welcome() -> impl Responder {
  let app_name = format!("welcome to {} API", env::var("APP_NAME").unwrap_or("actix-api".to_string()));

  response_json(
    "success".to_string(),
    app_name,
    vec![]
  )
}