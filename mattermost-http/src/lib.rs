use actix_web::{web, App, HttpServer, Responder};
use std::io;

async fn hello_world() -> impl Responder {
    "Hello World!"
}

pub async fn init_http_server() -> Result<(), io::Error> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let _ = HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .await
        .unwrap();

    sys.await
}
