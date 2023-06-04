use sqlx::Error;
use serde_json::Value;
use actix_web::{web::{self}, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::auth::*};

#[doc = "Verify user credentials and return token"]
pub async fn login(body: web::Json<Value>) -> impl Responder {
    let username = to_str(map_get("username", body.to_owned()));
    let password = to_str(map_get("password", body.to_owned()));

    if check_if_empty(username.to_owned()) || check_if_empty(password.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Please fill all fields".to_string(),
            vec![]
        )
    }

    let pool = connect_postgres().await;
    let user = match sqlx::query!("select * from client where username = $1", username)
        .fetch_one(&pool)
        .await {
            Ok(user) => user,
            Err(Error::RowNotFound) => return response_json(
                "failed".to_string(),
                "Account not found".to_string(),
                vec![]
            ),
            Err(_) => return response_json(
                "error".to_string(),
                "Something went wrong".to_string(),
                vec![]
            )
        };

    let verify_password = verify(password, &user.password).unwrap_or(false);

    if !verify_password {
        return response_json(
            "failed".to_string(),
            "Username or password is wrong".to_string(),
            vec![]
        )
    }

    let token_value = TokenStruct {
        id: user.id,
        username: user.username.clone(),
        role: user.role.clone(),
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_add(60 * 60),
    };
    
    let jwt_secret =  env::var("JWT_SECRET").unwrap_or(String::from("secret"));
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    let token = encode(&Header::default(), &token_value, &key).unwrap_or(String::from(""));

    let detail_user = convert_vec_to_values(vec![
        DetailUserStruct {
            id: user.id,
            username: user.username,
            role: user.role.clone(),
        }
    ]);

    response_json_with_cookie(
        "success".to_string(),
        "Successfully logged in".to_string(),
        detail_user,
        "set".to_string(),
        "auth_jwt_secret".to_string(),
        token.clone()
    )
}

#[doc = "Register new user"]
pub async fn register(body: web::Json<Value>) -> impl Responder {
    let username = to_str(map_get("username", body.to_owned()));
    let password = to_str(map_get("password", body.to_owned()));
    let role = to_str(map_get("role", body.to_owned()));

    if check_if_empty(username.to_owned()) || check_if_empty(password.to_owned()) || check_if_empty(role.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Please fill all fields".to_string(),
            vec![]
        )
    }

    let pool = connect_postgres().await;
    let check_user = sqlx::query!("select id, username from client where username = $1", username.to_owned())
        .fetch_optional(&pool)
        .await
        .unwrap();

    if check_user.is_some() {
        return response_json(
            "failed".to_string(),
            "Username already exists".to_string(),
            vec![]
        )
    }

    let hashed_password = hash(password, DEFAULT_COST).unwrap_or(String::from(""));

    match sqlx::query_as!(DetailUserStruct,
        "insert into client (username, password, role) values ($1, $2, $3) returning id, username, role",
        username.to_owned(),
        hashed_password,
        role.to_owned()
    ).fetch_one(&pool).await {
        Ok(data) => {
            let detail_user = convert_vec_to_values(vec![data]);

            response_json(
                "success".to_string(),
                "Successfully registered".to_string(),
                detail_user
            )
        },
        Err(_) => response_json(
            "error".to_string(),
            "Something went wrong".to_string(),
            vec![]
        )
    }
}

#[doc = "Logout user"]
pub async fn logout() -> impl Responder {
    response_json_with_cookie(
        "success".to_string(),
        "Successfully logged in".to_string(),
        vec![],
        "remove".to_string(),
        "auth_jwt_secret".to_string(),
        "".to_string()
    )
}