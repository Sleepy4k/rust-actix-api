use actix_web::{HttpRequest, Responder};

use crate::helpers::response::*;

#[doc = "Default route for all routes that are not defined"]
pub async fn fallback(req: HttpRequest) -> impl Responder {
    let path = req.path();
    let message = format!("route {} not found", path);

    response_json(
        "not found".to_string(),
        message,
        vec![]
    )
}