use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey,
    EncodingKey, Header, TokenData, Validation
};
use serde::{Deserialize, Serialize};

use crate::errors::AppResult;
use crate::services::user_repository::User;

pub const USER_COOKIE: &str = "RANT_USER";
pub const AUDIENCE: &str = "rant-app";
pub const ISSUER: &str = "rant-app";

pub fn get_current_timestamp() -> AppResult<f64> {
    let start = std::time::SystemTime::now();
    Ok(start.duration_since(std::time::UNIX_EPOCH)?.as_secs() as f64)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JWTBody {
    // Required claims for JWT
    iss: String,
    exp: f64,
    nbf: f64,
    sub: String,
    aud: String,

    // The user data.
    user: User,
}

impl JWTBody {
    pub fn new(user: User, time_to_live_seconds: f64) -> AppResult<Self> {
        Ok(Self {
            iss: ISSUER.into(),
            sub: user.id.clone(),
            aud: AUDIENCE.into(),
            nbf: get_current_timestamp()? as f64,
            exp: (get_current_timestamp()? + time_to_live_seconds) as f64,
            user,
        })
    }
}

#[derive(Clone)]
pub struct JWTService {
    secret: String,
    time_to_live_seconds: f64,
}

impl JWTService {
    pub fn new(secret: impl Into<String>, time_to_live_seconds: f64) -> Self {
        Self {
            secret: secret.into(),
            time_to_live_seconds
        }
    }

    pub fn to_jwt(&self, user: &User) -> AppResult<String> {
        let header = Header::new(Algorithm::HS512);
        Ok(encode(
            &header,
            &JWTBody::new(user.clone(), self.time_to_live_seconds)?,
            &EncodingKey::from_base64_secret(&self.secret)?
        )?)
    }

    pub fn get_user(&self, token: &str) -> AppResult<User> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.validate_aud = false;
        validation.set_issuer(&["rant-app"]);

        let token_data: TokenData<JWTBody> = decode(
            token,
            &DecodingKey::from_base64_secret(&self.secret)?,
            &validation
        )?;

        Ok(token_data.claims.user)
    }
}