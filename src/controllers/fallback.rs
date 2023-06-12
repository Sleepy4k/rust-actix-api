use serde_json::json;
use actix_web::{HttpRequest, Responder};

use crate::helpers::response::*;

#[doc = "Default route for all routes that are not defined"]
pub async fn fallback(req: HttpRequest) -> impl Responder {
  let path = req.path().to_string();
  let method = req.method().to_string();
  let data = vec![
    json!({
      "path": path,
      "method": method
    })
  ];

  response_json(
    "not found".to_string(),
    "Path not found".to_string(),
    data
  )
}