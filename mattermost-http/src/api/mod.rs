mod user;

pub(crate) mod v4 {
    use crate::api::user;
    use actix_web::web;

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/api/v4").configure(user::configure));
    }
}
