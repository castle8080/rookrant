use maud::{DOCTYPE, html, Markup};
use crate::views::header::header;
use crate::services::user_repository::User;

pub fn main_layout(user: Option<User>, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Rook Rant" }
            link rel="stylesheet" href="/css/rant.css" {}
        }
        body {
            (header(user))
            div .app-content {
                (content)
            }
        }
    }
}
