use maud::Markup;
use axum::{
    routing::{ get, post },
    Router,
    Form,
    extract::State,
};
use serde::{Deserialize, Serialize};
use log;

use crate::views;
use crate::services::state::ServicesState;
use crate::services::user_repository::UserRepositoryRef;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RantSaveRequest {
    rant: String
}

async fn rant_add_get() -> Markup {
    views::rant::add()
}

async fn rant_add_post(State(user_repository): State<UserRepositoryRef>, Form(req): Form<RantSaveRequest>) -> Markup {
    log::info!("There is a post to add: {:?}", req);

    let users = user_repository.get_users().await;
    log::debug!("Users: {:?}", users);

    views::rant::add()
}

pub fn get_routes() -> Router<ServicesState> {
    Router::new()
        .route("/rant_add", get(rant_add_get))
        .route("/rant_add", post(rant_add_post))
}