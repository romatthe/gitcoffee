use futures::{select, FutureExt};
use mattermost_http::init_http_server;

#[tokio::main]
async fn main() {
    let http_server = init_http_server();

    // Race the futures of the http server and shutdown handler
    select! {
        _ = http_server.fuse() => (),
        _ = shutdown_handler().fuse() => println!("Shutting down!"),
    }
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
