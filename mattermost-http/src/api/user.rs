use actix_web::{web, Responder, HttpResponse};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUserQuery {
    t: String,
    iid: String,
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    email: String,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    props: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_props: Option<NotifyProps>,
}

#[derive(Deserialize, Serialize)]
struct NotifyProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desktop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desktop_sound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<bool>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::post().to(create_user)));
}

async fn create_user(query: web::Query<CreateUserQuery>, item: web::Json<CreateUser>) -> impl Responder {
    HttpResponse::Created().json(item.0)
}
