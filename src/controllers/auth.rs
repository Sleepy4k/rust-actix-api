use chrono::Local;
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

    let start_time = Local::now().timestamp();

    if check_if_empty(username.to_owned()) || check_if_empty(password.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Please fill all fields".to_string(),
            vec![]
        )
    }

    let pool = connect_postgres().await;
    let user = match sqlx::query!("select * from client where username = $1 limit 1", username)
        .fetch_optional(&pool)
        .await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return response_json(
                    "failed".to_string(),
                    "Account not found".to_string(),
                    vec![]
                );
            }
            Err(_) => return response_json(
                "error".to_string(),
                "Something went wrong".to_string(),
                vec![]
            )
        };

    match verify(password, &user.password) {
        Ok(true) => (),
        Ok(false) => {
            return response_json(
                "failed".to_string(),
                "Username or password is wrong".to_string(),
                vec![]
            )
        }
        Err(_) => return response_json(
            "error".to_string(),
            "Something went wrong".to_string(),
            vec![]
        )
    };

    let token_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let token_value = TokenStruct {
        id: user.id,
        username: user.username.clone(),
        role: user.role.clone(),
        iat: token_time,
        exp: token_time.saturating_add(60 * 60 * 24 * 7),
    };
    
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"));
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    let token = encode(&Header::default(), &token_value, &key).unwrap_or_else(|_| String::new());

    let detail_user = convert_vec_to_values(vec![
        DetailUserStruct {
            id: user.id,
            username: user.username,
            role: user.role.clone(),
        }
    ]);

    let end_time = Local::now().timestamp();
    let duration = modified_duration(start_time, end_time);

    println!("{}", duration);

    response_json_with_cookie(
        "success".to_string(),
        "Successfully logged in".to_string(),
        detail_user,
        "set".to_string(),
        "auth_jwt_secret".to_string(),
        token,
    )
}

#[doc = "Register new user"]
pub async fn register(body: web::Json<Value>) -> impl Responder {
    let username = to_str(map_get("username", body.to_owned()));
    let password = to_str(map_get("password", body.to_owned()));
    let role = to_str(map_get("role", body.to_owned()));

    let start_time = Local::now().timestamp();

    if check_if_empty(username.to_owned()) || check_if_empty(password.to_owned()) || check_if_empty(role.to_owned()) {
        return response_json(
            "failed".to_string(),
            "Please fill all fields".to_string(),
            vec![]
        )
    }

    let pool = connect_postgres().await;

    match sqlx::query!("select id from client where username = $1", username.to_owned())
        .fetch_optional(&pool)
        .await {
            Ok(Some(_)) => {
                return response_json(
                    "failed".to_string(),
                    "Username already exists".to_string(),
                    vec![]
                )
            },
            Ok(None) => (),
            Err(_) => return response_json(
                "error".to_string(),
                "Something went wrong".to_string(),
                vec![]
            )
        };

    let hashed_password = hash(password, DEFAULT_COST).unwrap_or_else(|_| String::new());

    match sqlx::query_as!(DetailUserStruct,
        "insert into client (username, password, role) values ($1, $2, $3) returning id, username, role",
        username.to_owned(),
        hashed_password,
        role.to_owned()
    ).fetch_one(&pool).await {
        Ok(data) => {
            let detail_user = convert_vec_to_values(vec![data]);

            let end_time = Local::now().timestamp();
            let duration = modified_duration(start_time, end_time);
        
            println!("{}", duration);
        
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