use async_trait::async_trait;
use crate::data::User;

#[async_trait]
pub trait UsersModule {
    async fn list_users(&self) -> Vec<User>;
}