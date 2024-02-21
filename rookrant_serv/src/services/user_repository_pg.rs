use async_trait::async_trait;
use sqlx::PgPool;
use log;

use crate::errors::AppResult;
use crate::services::user_repository::{User, UserRepository};

pub struct UserRepositoryPg {
    db_pool: PgPool,
}

impl UserRepositoryPg {
    pub fn new(db_pool: PgPool) -> Self {
        UserRepositoryPg { db_pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPg {
    async fn get_users(&self) -> AppResult<Vec<User>> {
        log::info!("Getting users.");

        let users = sqlx::query_as::<_, User>(r"select * from rant_user")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(users)
    }
}
