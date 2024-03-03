use std::sync::Arc;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::errors::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> AppResult<Vec<User>>;
    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>>;
    async fn add_user(&self, user: &User) -> AppResult<()>;
}

pub type UserRepositoryRef = Arc<Box<dyn UserRepository + Send + Sync>>;

pub fn new_user_repository_ref(user_repository: impl UserRepository + Send + Sync + 'static) -> UserRepositoryRef {
    Arc::new(Box::new(user_repository))
}
