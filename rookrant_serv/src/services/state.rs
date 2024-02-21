use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use axum::extract::FromRef;

use crate::errors::{AppError, AppResult};

use crate::services::config_service::{
    ConfigServiceRetriever,
    ConfigServiceRef,
    new_config_service_ref
};
use crate::services::config_service_file::ConfigRetrieverFile;
use crate::services::config_service_chain::new_empty_chain;

use crate::services::user_repository::{
    UserRepositoryRef,
    new_user_repository_ref,
};
use crate::services::user_repository_pg::UserRepositoryPg;

#[derive(Clone)]
pub struct ServicesState {
    pub db_pool: PgPool,
    pub config_service: ConfigServiceRef,
    pub user_repository: UserRepositoryRef,
}

impl FromRef<ServicesState> for PgPool {
    fn from_ref(state: &ServicesState) -> PgPool {
        state.db_pool.clone()
    }
}

impl FromRef<ServicesState> for UserRepositoryRef {
    fn from_ref(state: &ServicesState) -> UserRepositoryRef {
        state.user_repository.clone()
    }
}

pub async fn configure(environment: &String) -> AppResult<ServicesState> {
    let config_service = create_config_service(environment)?;
    let conn_string = config_service.get_required_template("db_connection_string").await?;

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(conn_string.as_str())?;

    let user_repository = new_user_repository_ref(UserRepositoryPg::new(db_pool.clone()));

    Ok(ServicesState { db_pool, config_service, user_repository })
}

fn create_config_service(environment: &String) -> AppResult<ConfigServiceRef> {
    if environment == "local" {
        Ok(new_config_service_ref(ConfigServiceRetriever::new(
            new_empty_chain()
                .with(ConfigRetrieverFile::new(format!("etc/config/{environment}.json"))?)
                .with(ConfigRetrieverFile::new(format!("etc/secrets/{environment}.json"))?)
        )))
    }
    else {
        Err(AppError::ConfigurationError(format!("Unkonwn environment: {environment}")))
    }
}
