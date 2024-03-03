use axum::extract::FromRef;

use crate::url_constants::AUTH_LOGIN_COMPLETE;
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
use crate::services::user_repository_mongo::UserRepositoryMongo;

use crate::services::ms_oath_service::MSOAuthService;
use crate::services::jwt_service::JWTService;

#[derive(Clone)]
pub struct ServicesState {
    pub config_service: ConfigServiceRef,
    pub ms_oath_service: MSOAuthService,
    pub jwt_service: JWTService,
    pub user_repository: UserRepositoryRef,
}

impl FromRef<ServicesState> for UserRepositoryRef {
    fn from_ref(state: &ServicesState) -> UserRepositoryRef {
        state.user_repository.clone()
    }
}

impl FromRef<ServicesState> for MSOAuthService {
    fn from_ref(state: &ServicesState) -> MSOAuthService {
        state.ms_oath_service.clone()
    }
}

impl FromRef<ServicesState> for JWTService {
    fn from_ref(state: &ServicesState) -> JWTService {
        state.jwt_service.clone()
    }
}

pub async fn configure(environment: &String) -> AppResult<ServicesState> {
    let config_service = create_config_service(environment)?;
    let conn_string = config_service.get_required_template("db_connection_string").await?;

    let mongo_client = mongodb::Client::with_uri_str(conn_string).await?;
    let user_repository = new_user_repository_ref(UserRepositoryMongo::new(
        mongo_client,
        "rook",
        "users",
    ));

    let ms_oath_service = MSOAuthService::new(
        config_service.get_required("ms_oauth_app_id").await?,
        config_service.get_required("ms_oauth_secret").await?,
        config_service.get_required("ms_oauth_tennant_id").await?,
        AUTH_LOGIN_COMPLETE,
    );

    let jwt_service = JWTService::new(
        config_service.get_required("jwt_signing_secret").await?,
        config_service
            .get_required("jwt_expiration_seconds")
            .await?
            .parse::<f64>()?
    );

    Ok(ServicesState {
        config_service,
        ms_oath_service,
        user_repository,
        jwt_service,
    })
}

fn create_config_service(environment: &String) -> AppResult<ConfigServiceRef> {
    match environment.as_str() {
        "local"|"dev" => {
            Ok(new_config_service_ref(ConfigServiceRetriever::new(
                new_empty_chain()
                    .with(ConfigRetrieverFile::new(format!("etc/config/{environment}.json"))?)
                    .with(ConfigRetrieverFile::new(format!("etc/secrets/{environment}.json"))?)
            )))
        }
        _ => {
            Err(AppError::ConfigurationError(format!("Unkonwn environment: {environment}")))
        }
    }
}
