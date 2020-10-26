use super::*;
use actix_web::{web, Responder, HttpResponse};
use mattermost_core::modules::UsersModule;
use std::ops::Deref;
use crate::context::UserContext;

pub(super) async fn list_users(users: web::Data<UserContext>) -> impl Responder {
    let all_users = users.list_users().await;

    HttpResponse::Ok().json(all_users)
}

pub(super) async fn create_user(query: web::Query<CreateUserQuery>, item: web::Json<CreateUser>) -> impl Responder {
    // if let Some(token_id) = &query.t {
    //     // TODO: create_user_with_token
    // } else if let Some(invite_id) = &query.iid {
    //     // TODO: create_user_with_invite_id
    // // } else if I am a sysadmin {
    // //     // TODO: Check if I'm a sysadmin
    // //     ruser, err = c.App.CreateUserAsAdmin(user, redirect)
    // } else {
    //     // TODO: create_user_from_signup
    // }

    HttpResponse::Created().json(item.0)
}
