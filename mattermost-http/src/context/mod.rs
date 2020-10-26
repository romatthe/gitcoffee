use async_trait::async_trait;
use mattermost_core::modules::UsersModule;
use mattermost_core::data::User;
use sqlx::{Pool, Database, Sqlite};

type DbPool = Pool<Sqlite>;

#[derive(Clone)]
pub struct UserContext {
    pool: DbPool
}

impl UserContext {
    pub fn new(pool: DbPool) -> Self {
        UserContext {
            pool
        }
    }
}

#[async_trait]
impl UsersModule for UserContext {
    async fn list_users(&self) -> Vec<User> {
        vec![]
    }
}