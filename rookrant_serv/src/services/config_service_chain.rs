use async_trait::async_trait;
use crate::errors::AppResult;
use crate::services::config_service::ConfigRetriever;

pub struct ConfigRetrieverChain<T1, T2>
    where T1: ConfigRetriever + Send, T2: ConfigRetriever + Send
{
    first: T1,
    remaining: T2,
}

pub struct ConfigRetrieverEmpty {}

#[async_trait]
impl ConfigRetriever for ConfigRetrieverEmpty {
    async fn get(&self, _key: &str) -> AppResult<Option<String>> {
        Ok(None)
    }
}

pub fn new_empty_chain() -> ConfigRetrieverChain<ConfigRetrieverEmpty, ConfigRetrieverEmpty> {
    ConfigRetrieverChain { first: ConfigRetrieverEmpty {}, remaining: ConfigRetrieverEmpty { }}
}

impl<T1, T2> ConfigRetrieverChain<T1, T2> 
    where T1: ConfigRetriever + Send + Sync, T2: ConfigRetriever + Send + Sync
{
    pub fn with<T3>(self, new_first: T3) -> ConfigRetrieverChain<T3, ConfigRetrieverChain<T1, T2>>
        where T3: ConfigRetriever + Send
    {
        ConfigRetrieverChain { first: new_first, remaining: self }
    }
}

#[async_trait]
impl<T1, T2> ConfigRetriever for ConfigRetrieverChain<T1, T2>
    where T1: ConfigRetriever + Send + Sync, T2: ConfigRetriever + Send + Sync
{
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        match self.first.get(key).await {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => self.remaining.get(key).await,
            e => e
        }
    }
}
