mod api;

use actix_web::{App, HttpServer};
use api::v4;
use std::io;

pub async fn init_http_server() -> Result<(), io::Error> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let _ = HttpServer::new(|| App::new().configure(v4::configure))
        .bind("0.0.0.0:8000")?
        .run()
        .await?;

    sys.await
}
