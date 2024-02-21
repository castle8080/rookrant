use std::path::Path;
use std::fs::File;
use std::io;

use async_trait::async_trait;
use serde_json;

use crate::errors::{AppResult, AppError};
use crate::services::config_service::ConfigRetriever;

#[derive(Debug, Clone)]
pub struct ConfigRetrieverFile {
    data: serde_json::Map<String, serde_json::Value>,
}

impl ConfigRetrieverFile {
    pub fn new(file_name: impl AsRef<Path>) -> AppResult<Self> {
        let buf = io::BufReader::new(File::open(file_name.as_ref())?);
        match serde_json::from_reader(buf)? {
            serde_json::Value::Object(data) => {
                Ok(Self { data })
            }
            _ => {
                Err(AppError::ParseError(format!("Invalid json content for configuration.")))
            }
        }
    }
}

#[async_trait]
impl ConfigRetriever for ConfigRetrieverFile {
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let val = match self.data.get(key) {
            Option::Some(serde_json::Value::String(v)) => Some(v.clone()),
            _ => Option::None
        };

        Ok(val)
    }
}
