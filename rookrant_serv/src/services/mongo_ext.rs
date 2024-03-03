use async_trait::async_trait;

use mongodb::Cursor;
use serde::de::DeserializeOwned;

use crate::errors::{AppResult, AppError};

// Extentions to make working with Cursor easier.

#[async_trait]
pub trait MongoCursorExtensions<T> {
    async fn to_vec(&mut self) -> AppResult<Vec<T>>;

    async fn single(&mut self) -> AppResult<T>;

    async fn single_or_none(&mut self) -> AppResult<Option<T>>;
}

#[async_trait]
impl<T> MongoCursorExtensions<T> for Cursor<T> where T: DeserializeOwned + Send {

    async fn to_vec(&mut self) -> AppResult<Vec<T>> {
        let mut items: Vec<T> = Vec::new();
        while self.advance().await? {
            items.push(self.deserialize_current()?);
        }
        Ok(items)
    }

    async fn single(&mut self) -> AppResult<T> {
        Ok(self.single_or_none()
            .await?
            .ok_or(AppError::DatabaseError(format!("No result found for query.")))?)
    }

    async fn single_or_none(&mut self) -> AppResult<Option<T>> {
        if !self.advance().await? {
            return Ok(Option::None);
        }

        let item = self.deserialize_current()?;

        if self.advance().await? {
            return Err(AppError::DatabaseError(format!("More than 1 result found for query.")));
        }

        Ok(Some(item))
    }
}
