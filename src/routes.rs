use super::controllers::{get_users, index};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .route("/", web::get().to(index))
        .route("/users", web::get().to(get_users));
}
