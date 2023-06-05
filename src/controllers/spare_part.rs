use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::spare_part::*};

#[doc = "Get all spare parts"]
pub async fn get_spare_part() -> impl Responder {
    let pool = connect_postgres().await;
    let data = sqlx::query_as!(SparePartStruct, "select * from spare_part")
        .fetch_all(&pool)
        .await
        .unwrap();

    let result = convert_vec_to_values(data);

    response_json(
        "success".to_string(),
        "Successfully retrieved spare part".to_string(),
        result
    )
}

#[doc = "Add new spare part"]
pub async fn add_spare_part(body: web::Json<Value>) -> impl Responder {
    let name = to_str(map_get("name", body.to_owned()));
    let price = to_i32(map_get("price", body.to_owned()));

    if check_if_empty(name.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Please fill all fields".to_string(),
            vec![]
        )
    }

    let pool = connect_postgres().await;
    let data = sqlx::query_as!(SparePartStruct, "insert into spare_part (name, price) values ($1, $2) returning *", name, price)
        .fetch_all(&pool)
        .await
        .unwrap();

    let result = convert_vec_to_values(vec![data]);

    response_json(
        "success".to_string(),
        "Successfully added spare part".to_string(),
        result
    )
}

#[doc = "Find spare part by id"]
pub async fn find_spare_part(arg: web::Path<i32>) -> impl Responder {
    let id = arg.to_owned();

    let pool = connect_postgres().await;
    let data = sqlx::query_as!(SparePartStruct, "select * from spare_part where id = $1", id)
        .fetch_all(&pool)
        .await
        .unwrap();


    let result = convert_vec_to_values(data);

    if check_if_empty(result.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Spare part not found".to_string(),
            vec![]
        )
    }

    response_json(
        "success".to_string(),
        "Successfully retrieved spare part".to_string(),
        result
    )
}

#[doc = "Update spare part by id"]
pub async fn update_spare_part(body: web::Json<Value>, arg: web::Path<i32>) -> impl Responder {
    let id = arg.to_owned();
    let name = to_str(map_get("name", body.to_owned()));
    let price = to_i32(map_get("price", body.to_owned()));

    let pool = connect_postgres().await;
    let data = sqlx::query_as!(SparePartStruct, "update spare_part set name = $1, price = $2 where id = $3 returning *", name, price, id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let result = convert_vec_to_values(data);

    if check_if_empty(result.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Spare part not found".to_string(),
            vec![]
        )
    }

    response_json(
        "success".to_string(),
        "Successfully updated spare part".to_string(),
        result
    )
}

#[doc = "Delete spare part by id"]
pub async fn delete_spare_part(arg: web::Path<i32>) -> impl Responder {
    let id = arg.to_owned();

    let pool = connect_postgres().await;
    let data = sqlx::query_as!(SparePartStruct, "delete from spare_part where id = $1 returning *", id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let result = convert_vec_to_values(data);

    if check_if_empty(result.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Spare part not found".to_string(),
            vec![]
        )
    }

    response_json(
        "success".to_string(),
        "Successfully deleted spare part".to_string(),
        result
    )
}