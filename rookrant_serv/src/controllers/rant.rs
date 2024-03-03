use maud::Markup;
use axum::{
    extract::State, routing::{ get, post }, Extension, Form, Router
};
use serde::{Deserialize, Serialize};
use log;

use crate::{services::user_repository::User, views};
use crate::services::state::ServicesState;
use crate::services::user_repository::UserRepositoryRef;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RantSaveRequest {
    rant: String
}

async fn rant_add_get(Extension(user): Extension<Option<User>>) -> Markup {
    views::rant::add(user)
}

async fn rant_add_post(
    Extension(user): Extension<Option<User>>,
    State(user_repository): State<UserRepositoryRef>,
    Form(req): Form<RantSaveRequest>)
    -> Markup
{
    log::info!("There is a post to add: {:?}", req);

    let users = user_repository.get_users().await;
    log::debug!("Users: {:?}", users);

    views::rant::add(user)
}

pub fn get_routes() -> Router<ServicesState> {
    Router::new()
        .route("/rant_add", get(rant_add_get))
        .route("/rant_add", post(rant_add_post))
}