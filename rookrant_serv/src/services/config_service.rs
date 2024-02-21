use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use async_recursion::async_recursion;

use crate::errors::{AppError, AppResult};

/// Retrieve configuration values.
#[async_trait]
pub trait ConfigRetriever {
    async fn get(&self, key: &str) -> AppResult<Option<String>>;
}

#[async_trait]
pub trait ConfigService {
    async fn get(&self, key: &str) -> AppResult<Option<String>>;
    async fn get_required(&self, key: &str) -> AppResult<String>;
    async fn get_required_template(&self, key: &str) -> AppResult<String>;
}

pub struct ConfigServiceRetriever<T> where T: ConfigRetriever + Send {
    retriever: T,
}

impl<T> ConfigServiceRetriever<T> where T: ConfigRetriever + Send {
    pub fn new(retriever: T) -> Self {
        Self { retriever }
    }
}

#[async_trait]
impl<T> ConfigService for ConfigServiceRetriever<T> where T: ConfigRetriever + Send + Sync {

    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        self.retriever.get(key).await
    }

    async fn get_required(&self, key: &str) -> AppResult<String>
    {
        match self.get(key).await {
            Ok(Some(s)) => Ok(s),
            Ok(None) => Err(AppError::ConfigurationError(format!("Missing configuration key for {key}."))),
            Err(e) => Err(e)
        }
    }

    async fn get_required_template(&self, key: &str) -> AppResult<String> {
        let mut visiting: HashSet<String> = HashSet::new();
        Ok(get_required_template_recur(self, key, &mut visiting).await?)
    }

}

#[async_recursion]
async fn get_required_template_recur<T>(_self: &ConfigServiceRetriever<T>, key: &str, visiting: &mut HashSet<String>)
    -> AppResult<String>
    where T: ConfigRetriever + Send + Sync
{
    if visiting.contains(key) {
        return Err(AppError::ConfigurationError(format!("Recursive configuration key found: {key}")));
    }
    visiting.insert(key.to_string());


    let template_text = _self.get_required(key).await?;
    let template = leon::Template::parse(&template_text)?;

    let mut vars: HashMap<String, String> = HashMap::new();

    for sub_key in template.keys() {
        let sub_key = sub_key.to_string();
        let var_val = get_required_template_recur(_self, &sub_key, visiting).await?;
        vars.insert(sub_key, var_val);
    }

    visiting.remove(key);
    Ok(template.render(&vars)?)
}

pub type ConfigServiceRef = Arc<Box<dyn ConfigService + Send + Sync>>;

pub fn new_config_service_ref(config_service: impl ConfigService + Send + Sync + 'static) -> ConfigServiceRef {
    Arc::new(Box::new(config_service))
}