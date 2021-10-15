use super::models::{IndexResponse, UserResponse};
use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    let response = IndexResponse {
        message: "Hello World!!".to_string(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_users() -> impl Responder {
    let users = vec![UserResponse {
        id: 1,
        name: "alice".to_string(),
    }];

    HttpResponse::Ok().json(users)
}
