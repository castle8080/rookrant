use crate::{services::user_repository::User, views};

use maud::Markup;
use axum::{
    routing::get, Extension, Router
};

use crate::services::state::ServicesState;

async fn index(Extension(user): Extension<Option<User>>) -> Markup {
    views::welcome::index(user)
}

pub fn get_routes() -> Router<ServicesState> {
    Router::new()
        .route("/", get(index))
}