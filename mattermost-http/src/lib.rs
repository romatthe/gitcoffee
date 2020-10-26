mod api;
pub mod context;

use actix_web::{App, HttpServer, web};
use api::v4;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::context::UserContext;

pub struct MattermostHttpServer {
    users: UserContext
}

impl MattermostHttpServer {
    pub fn init(users: UserContext) -> Self {
        MattermostHttpServer {
            users
        }
    }

    pub async fn run<A: ToSocketAddrs>(self, addr: A) -> Result<(), io::Error> {
        let users = self.users;

        let local = tokio::task::LocalSet::new();
        let sys = actix_rt::System::run_in_tokio("server", &local);
        let _ = HttpServer::new(move ||
            App::new()
                .data(users.clone())
                .configure(v4::configure)
            )
            .bind(addr)?
            .run()
            .await?;

        sys.await
    }
}
