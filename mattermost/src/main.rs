use futures::{select, FutureExt};
use mattermost_http::MattermostHttpServer;
use sqlx::sqlite::SqlitePoolOptions;
use mattermost_http::context::UserContext;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:").await?;


    let users = UserContext::new(pool.clone());

    let http_server = MattermostHttpServer::init(users).run("0.0.0.0:8000");

    // Race the futures of the http server and shutdown handler
    select! {
        _ = http_server.fuse() => (),
        _ = shutdown_handler().fuse() => println!("Shutting down!"),
    }

    Ok(())
}

#[cfg(unix)]
async fn shutdown_handler() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    select! {
        _ = sigterm.recv().fuse() => (),
        _ = sigint.recv().fuse() => (),
    }

    Ok(())
}

#[cfg(windows)]
async fn shutdown_handler() -> Result<(), Box<dyn std::error::Error>> {
    tokio::signal::ctrl_c().await?;

    Ok(())
}
