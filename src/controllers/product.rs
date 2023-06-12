use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::product::*};

#[doc = "Get all products"]
pub async fn get_product() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ProductStruct, "select * from product")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved product".to_string(),
    result
  )
}

#[doc = "Add new product"]
pub async fn add_product(body: web::Json<Value>) -> impl Responder {
  let name = to_str(map_get("name", body.to_owned()));
  let price = to_i32(map_get("price", body.to_owned()));
  let amount = to_i32(map_get("amount", body.to_owned()));

  if check_if_empty(name.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let pool = connect_postgres().await;

  match sqlx::query!("select * from product where name = $1 limit 1", name.clone())
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => {
        return response_json(
          "failed".to_string(),
          "Product already exists".to_string(),
          vec![]
        )
      }
      Ok(None) => (),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let data = sqlx::query_as!(ProductStruct, "insert into product (name, price, amount) values ($1, $2, $3) returning *", name, price, amount)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(vec![data]);

  response_json(
    "success".to_string(),
    "Successfully added product".to_string(),
    result
  )
}

#[doc = "Find product by id"]
pub async fn find_product(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ProductStruct, "select * from product where id = $1", id)
    .fetch_all(&pool)
    .await
    .unwrap();


  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Product not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully retrieved product".to_string(),
    result
  )
}

#[doc = "Update product by id"]
pub async fn update_product(body: web::Json<Value>, arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();
  let name = to_str(map_get("name", body.to_owned()));
  let price = to_i32(map_get("price", body.to_owned()));
  let amount = to_i32(map_get("amount", body.to_owned()));

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ProductStruct, "update product set name = $1, price = $2, amount = $3 where id = $4 returning *", name, price, amount, id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Product not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully updated product".to_string(),
    result
  )
}

#[doc = "Delete product by id"]
pub async fn delete_product(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ProductStruct, "delete from product where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Product not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully deleted product".to_string(),
    result
  )
}