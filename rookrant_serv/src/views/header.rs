use maud::{html, Markup};
use crate::services::user_repository::User;

pub fn header(user: Option<User>) -> Markup {
    html! {
        div .app-header {
            span .app-header-main-links {
                a href="/" { "Rant" }
            }
            (user_links(user))
        }
    }
}

fn user_links(user: Option<User>) -> Markup {
    html! {
        div .app-header-user-links {
            @match user {
                None => {
                    "[" a href="/auth/login" { "Login" } "]"
                },
                Some(_) => {
                    "[" a href="/auth/logout" { "Logout" } "]"
                },
            }
        }
    }
}