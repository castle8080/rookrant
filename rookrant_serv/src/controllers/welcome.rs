use crate::views;

use maud::Markup;
use axum::{
    routing::get,
    Router,
};

use crate::services::state::ServicesState;

async fn index() -> Markup {
    views::welcome::index()
}

pub fn get_routes() -> Router<ServicesState> {
    Router::new()
        .route("/", get(index))
}