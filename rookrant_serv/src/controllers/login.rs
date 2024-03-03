use maud::Markup;

use axum::{Router, Extension};
use axum::extract::{State, Request};
use axum::routing::{get, post};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::CookieJar;

use cookie::CookieBuilder;

use serde::{Serialize, Deserialize};

use crate::errors::AppResult;
use crate::services::state::ServicesState;
use crate::services::ms_oath_service::MSOAuthService;
use crate::services::jwt_service::{JWTService, USER_COOKIE};
use crate::services::user_repository::{User, UserRepositoryRef};
use crate::url_constants::{
    AUTH_LOGIN, AUTH_LOGIN_START, AUTH_LOGIN_COMPLETE,
    AUTH_LOGOUT, AUTH_LOGOUT_PROVIDER
};
use crate::views;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AuthCallbackRequest {
    pub code: String,
    pub state: String,
}

async fn login(Extension(user): Extension<Option<User>>) -> Markup {
    views::login::login(user)
}

async fn login_start(State(oauth_service): State<MSOAuthService>, mut request: Request) -> AppResult<Redirect> {
    let (auth_url, _csrf_token) = oauth_service.login_start(&mut request)?;
    Ok(Redirect::to(auth_url.as_str()))
}

async fn login_complete(
    State(oauth_service): State<MSOAuthService>,
    State(user_repository): State<UserRepositoryRef>,
    State(jwt_service): State<JWTService>,
    cookies: CookieJar,
    mut request: Request) -> AppResult<impl IntoResponse>
{
    let claims = oauth_service.process_auth_response(&mut request).await?;

    // Get and possibly create user.
    let user = match user_repository.get_user_by_id(&claims.oid).await? {
        Some(user) => {
            user
        },
        None => {
            let user = claims.to_user();
            user_repository.add_user(&user).await?;
            user
        }
    };

    let token = jwt_service.to_jwt(&user)?;

    let cookies = cookies.add(
        CookieBuilder::new(USER_COOKIE, token)
            .path("/")
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .expires(cookie::time::OffsetDateTime::now_utc() + cookie::time::Duration::days(365))
            .build()
    );
    Ok((
        cookies,
        views::login::login_complete(Some(user))
    ))
}

async fn logout(cookies: CookieJar) -> AppResult<impl IntoResponse> {
    let cookies = cookies.remove(
        CookieBuilder::new(USER_COOKIE, "")
            .secure(true)
            .path("/")
            .same_site(cookie::SameSite::Strict)
            .build()
    );

    Ok((cookies, Redirect::to(AUTH_LOGOUT_PROVIDER)))
}

async fn logout_of_provider(State(oauth_service): State<MSOAuthService>, mut request: Request) -> AppResult<impl IntoResponse> {
    let logout_url = oauth_service.logout(&mut request, AUTH_LOGIN)?;
    Ok(Redirect::to(logout_url.as_str()))
}

pub fn get_routes() -> Router<ServicesState> {
    Router::new()
        .route(AUTH_LOGIN, get(login))
        .route(AUTH_LOGIN_START, post(login_start))
        .route(AUTH_LOGIN_COMPLETE, get(login_complete))
        .route(AUTH_LOGOUT, get(logout))
        .route(AUTH_LOGOUT_PROVIDER, get(logout_of_provider))
}