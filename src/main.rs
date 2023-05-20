use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};

use actix::{model::AppState, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let todo_db = AppState::init();
    let app_data = web::Data::new(todo_db);

    println!("Starting server at: http://localhost:7004");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .configure(routes::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("localhost", 7004))?
    .run()
    .await
}
