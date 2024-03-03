use maud::{html, Markup};
use crate::views::main_layout::main_layout;
use crate::services::user_repository::User;

pub fn index(user: Option<User>) -> Markup {
    main_layout(user, html!(
        a href="/rant_add" { "Start Ranting" }
    ))
}