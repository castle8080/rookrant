use std::collections::HashMap;

// Utility to integrate with Microsoft EntraID using oauth2.
use axum::http::Uri;
use axum::extract::{Query, Request};
use axum::RequestExt;
use base64::Engine;
use log;

use serde::{Deserialize, Serialize};

use oauth2::url::Url;
use oauth2::{CsrfToken, Scope};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, RedirectUrl, TokenUrl
};

use crate::errors::{AppError, AppResult};
use crate::services::user_repository::User;

// The id_token field is needed to get information about the user when
// exchanging for a code.

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MSExtraTokenFields {
    pub id_token: String
}

impl oauth2::ExtraTokenFields for MSExtraTokenFields {}

pub type MSClient = oauth2::Client<
    oauth2::basic::BasicErrorResponse,
    oauth2::StandardTokenResponse<MSExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::basic::BasicTokenType,
    oauth2::basic::BasicTokenIntrospectionResponse,
    oauth2::StandardRevocableToken,
    oauth2::basic::BasicRevocationErrorResponse,
>;

// The fields in the id_token body.

//https://learn.microsoft.com/en-us/entra/identity-platform/id-token-claims-reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub name: String,
    pub preferred_username: String,
    pub oid: String,
    pub email: Option<String>,
}

impl TokenClaims {
    pub fn to_user(&self) -> User {
        User {
            id: self.oid.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            created: chrono::Utc::now(),
        }
    }
}

// The query param values which are set back on redirect.

#[derive(Serialize, Deserialize)]
struct AuthCallbackRequest {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct MSOAuthService {
    app_id: String,
    client_secret: String,
    tennant_id: String,
    callback_path: String,
}

impl MSOAuthService {
    pub fn new(
        app_id: impl Into<String>,
        client_secret: impl Into<String>,
        tennant_id: impl Into<String>,
        callback_path: impl Into<String>
    ) -> Self {
        Self {
            app_id: app_id.into(),
            client_secret: client_secret.into(),
            tennant_id: tennant_id.into(),
            callback_path: callback_path.into(),
        }
    }

    pub fn login_start(&self, request: &mut Request) -> AppResult<(Url, CsrfToken)> {
        let client = self
            .get_base_client()?
            .set_redirect_uri(self.get_redirect_url(request, &self.callback_path)?);
    
        // Generate the full authorization URL.
        Ok(client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("offline_access".to_string()))
            .url()
        )
    }

    pub fn logout(&self, request: &mut Request, on_logout_complete: &str) -> AppResult<Url> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert(
            "post_logout_redirect_uri",
            self.get_redirect_url(request, on_logout_complete)?.to_string()
        );

        let logout_uri = Url::parse_with_params(
            "https://login.microsoftonline.com/common/oauth2/v2.0/logout",
            params
        )?;

        Ok(logout_uri)
    }

    pub async fn process_auth_response(&self, request: &mut Request) -> AppResult<TokenClaims> {
        // TODO: Should add check for CSRF
        // TODO: Should also use Pkce code challenge.

        let client = self.get_base_client()?
            .set_redirect_uri(self.get_redirect_url(request, &self.callback_path)?);

        let Query(auth_params) = request
            .extract_parts::<Query<AuthCallbackRequest>>()
            .await
            .map_err(|e| { AppError::RequestInputError(format!("{}", e)) })?;

        let token_result =
            client
                .exchange_code(AuthorizationCode::new(auth_params.code.to_string()))
                .request_async(async_http_client)
                .await
                .map_err(|e| {
                    log::error!("Unable to authenticate: {:?}", e);
                    AppError::AuthenticationError(format!("Unable to authenticate user."))
                })?;

        let id_token = &token_result.extra_fields().id_token;
        let claims = self.parse_id_token(id_token)?;

        Ok(claims)
    }

    fn parse_id_token(&self, id_token: &str) -> AppResult<TokenClaims> {
        let id_token = id_token;
        let parts = id_token.split('.').collect::<Vec<&str>>();

        // TODO: The claims should be validated as should be the signature.

        let payload_part = *parts
            .get(1)
            .ok_or(AppError::AuthenticationError(format!("Invalid JWT token.")))?;

        let claims = serde_json::from_slice::<TokenClaims>(
            &base64::engine::general_purpose::STANDARD_NO_PAD.decode(payload_part)?
        )?;
        
        Ok(claims)
    }

    fn get_redirect_url(&self, request: &mut Request, return_path: impl AsRef<str>) -> AppResult<RedirectUrl> {
        let incoming_path = request.uri();
        let scheme = incoming_path.scheme()
            .ok_or(AppError::HttpError(String::from("Missing scheme")))?.as_str();
    
        let authority = incoming_path.authority()
            .ok_or(AppError::HttpError(String::from("Missing authority")))?.as_str();

        let redirect_uri = Uri::builder()
            .scheme(scheme)
            .authority(authority)
            .path_and_query(return_path.as_ref())
            .build()?;

        Ok(RedirectUrl::new(redirect_uri.to_string())?)
    }

    fn get_base_client(&self) -> AppResult<MSClient> {
        let login_uri_prefix = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/", self.tennant_id);
        Ok(
            MSClient::new(
                ClientId::new(self.app_id.clone()),
                Some(ClientSecret::new(self.client_secret.clone())),
                AuthUrl::new(format!("{}/authorize", login_uri_prefix))?,
                Some(TokenUrl::new(format!("{}/token", login_uri_prefix))?),
            )
        )
    }
}

