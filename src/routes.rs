use actix_web::web::{self};

use crate::controllers::*;

pub fn config(conf: &mut web::ServiceConfig) {
    conf
        // welcome route
        .route("/", web::route().to(welcome))

        // product route
        .route("/product", web::get().to(get_product))
        .route("/product", web::post().to(add_product))
        .route("/product/{id}", web::get().to(find_product))
        .route("/product/{id}", web::put().to(update_product))
        .route("/product/{id}", web::delete().to(delete_product))

        // missing route
        .default_service(web::route().to(fallback))
    ;
}