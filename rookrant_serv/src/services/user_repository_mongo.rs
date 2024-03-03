use async_trait::async_trait;

use mongodb::bson::doc;
use mongodb::{Client, Collection};

use crate::errors::AppResult;
use crate::services::user_repository::{User, UserRepository};

use super::mongo_ext::MongoCursorExtensions;

pub struct UserRepositoryMongo {
    mongo_client: Client,
    database_name: String,
    collection_name: String,
}

impl UserRepositoryMongo {
    pub fn new(mongo_client: Client, database_name: impl AsRef<str>, collection_name: impl AsRef<str>) -> Self {
        Self {
            mongo_client,
            database_name: database_name.as_ref().into(),
            collection_name: collection_name.as_ref().into(),
        }
    }

    fn users_collection(&self) -> Collection<User> {
        let db = self.mongo_client.database(&self.database_name);
        db.collection::<User>(&self.collection_name)
    }
}

#[async_trait]
impl UserRepository for UserRepositoryMongo {
    
    async fn get_users(&self) -> AppResult<Vec<User>> {
        Ok(self
            .users_collection()
            .find(doc! {}, None)
            .await?
            .to_vec()
            .await?
        )
    }

    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        Ok(self
            .users_collection()
            .find(doc! { "id": id }, None)
            .await?
            .single_or_none()
            .await?
        )
    }

    async fn add_user(&self, user: &User) -> AppResult<()> {
        let _ = self.users_collection().insert_one(user, None).await?;
        Ok(())
    }
}
