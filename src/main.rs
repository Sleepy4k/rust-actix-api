use std::env;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{error, http::header, web::JsonConfig, App, HttpResponse, HttpServer, middleware::{Logger, DefaultHeaders}};

use actix_api::*;
extern crate argon2;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "actix_api=debug,actix_web=info");
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("APP_PORT")
        .expect("no environment variable set for \"ENV STATUS\"")
        .parse::<u16>()
        .unwrap_or(8080);

    let hostname = env::var("APP_HOST")
        .expect("no environment variable set for \"ENV STATUS\"")
        .parse::<String>()
        .unwrap_or("localhost".to_string());

    let server_url = format!("{}:{}", hostname, port);

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        let json_config = JsonConfig::default()
            .limit(104857600)
            .error_handler(|err, _req| {
                error::InternalError::from_response(
                    err,
                    HttpResponse::Conflict().finish(),
                )
                .into()
            });

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Service-Version", "1.0.0")))
            .wrap(cookie::CheckCookie)
            .app_data(json_config)
            .configure(routes::config)
    })
    .bind(server_url.to_owned())?
    .run();
    
    println!("Server running at http://{}/", server_url);

    server.await?;

    Ok(())
}
