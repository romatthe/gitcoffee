use actix_web::{web, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::get().to(hello_world)));
}

async fn hello_world() -> impl Responder {
    "Hello World!"
}
