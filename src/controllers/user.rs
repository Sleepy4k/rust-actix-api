use actix_web::Responder;

use crate::{helpers::{response::*, database::connect_postgres, parse::*}, structs::auth::*};

#[doc = "Get all users"]
pub async fn get_user() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(DetailUserStruct, "select id, username, role from client")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved user".to_string(),
    result
  )
}