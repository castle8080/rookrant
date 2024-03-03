use base64::Engine;
use axum::{
    routing::get,
    Router,
};

use rand::{rngs::OsRng, RngCore};

use crate::services::state::ServicesState;

// Generates a random byte array secret
async fn utils_generate_secret() -> String {
    let mut rng = OsRng;
    let mut buffer = vec![0u8; 64];
    rng.fill_bytes(&mut buffer);

    let encoded_secret = base64::engine::general_purpose::STANDARD.encode(&buffer);
    String::from(format!("Secret: {}", encoded_secret))
}

pub fn get_routes() -> Router<ServicesState> {
    Router::new()
        .route("/utils/generate_secret", get(utils_generate_secret))
}