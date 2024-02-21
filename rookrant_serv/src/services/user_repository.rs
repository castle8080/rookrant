use std::sync::Arc;
use async_trait::async_trait;

use crate::errors::AppResult;

#[derive(Debug, Clone)]
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> AppResult<Vec<User>>;
}

pub type UserRepositoryRef = Arc<Box<dyn UserRepository + Send + Sync>>;

pub fn new_user_repository_ref(user_repository: impl UserRepository + Send + Sync + 'static) -> UserRepositoryRef {
    Arc::new(Box::new(user_repository))
}
