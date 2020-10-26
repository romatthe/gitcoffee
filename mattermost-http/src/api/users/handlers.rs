use super::*;
use actix_web::{web, Responder, HttpResponse};

pub(super) async fn create_user(query: web::Query<CreateUserQuery>, item: web::Json<CreateUser>) -> impl Responder {
    HttpResponse::Created().json(item.0)
}
