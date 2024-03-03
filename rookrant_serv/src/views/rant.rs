use maud::{html, Markup};
use crate::views::main_layout::main_layout;
use crate::services::user_repository::User;

pub fn add(user: Option<User>) -> Markup {
    main_layout(user, html!(
        h1 { "Rant About Something" }
        form method="POST" action="/rant_add" {
            textarea name="rant" rows="10" cols="80" {
            }
            br {}
            input type="submit" value="Add" { }
        }
    ))
}

