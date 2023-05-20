use actix_web::web;
use crate::{get_welcome, get_todo, create_todo, find_todo, update_todo, delete_todo};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        // Welcome routes
        .service(get_welcome)

        // Todo routes
        .service(get_todo)
        .service(create_todo)
        .service(find_todo)
        .service(update_todo)
        .service(delete_todo);

    conf.service(scope);
}