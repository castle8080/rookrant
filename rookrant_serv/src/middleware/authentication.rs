use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};

use axum_extra::extract::cookie::CookieJar;
use lazy_regex::regex;

use crate::errors::AppResult;
use crate::services::jwt_service::{JWTService, USER_COOKIE};
use crate::services::state::ServicesState;
use crate::services::user_repository::User;
use crate::url_constants::AUTH_LOGIN;

pub async fn authentication_layer(
    State(services_state): State<ServicesState>,
    mut request: Request,
    next: Next) -> Response
{
    match get_user_from_cookies(&services_state.jwt_service, &request) {
        Ok(None) => {
            request.extensions_mut().insert(None::<User>);
            if allowed_no_auth(&request) {
                return next.run(request).await.into_response();
            }
            else {
                return Redirect::to(AUTH_LOGIN).into_response();
            }
        },
        Ok(Some(user)) => {
            request.extensions_mut().insert(Some(user));
            return next.run(request).await.into_response();
        },
        Err(e) => {
            log::error!("Error getting user: {:?}", e);
            request.extensions_mut().insert(None::<User>);
            if allowed_no_auth(&request) {
                return next.run(request).await.into_response();
            }
            else {
                return Redirect::to(AUTH_LOGIN).into_response();
            }
        }
    }
}

fn allowed_no_auth(request: &Request) -> bool {
    let allowed_no_auth_regex = regex!("^/(auth|script|css|images|welcome)");
    allowed_no_auth_regex.is_match(request.uri().path())
}

fn get_user_from_cookies(jwt_service: &JWTService, request: &Request) -> AppResult<Option<User>> {
    let cookies = CookieJar::from_headers(request.headers());
    match cookies.get(USER_COOKIE) {
        Some(user_cookie) => {
            Ok(Some(jwt_service.get_user(user_cookie.value())?))
        },
        None => {
            Ok(None)
        }
    }
}