use maud::{DOCTYPE, html, Markup};

pub fn index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Rook Rant" }
        }
        body {
            h1 { "Rook Rant" }
            ul {
                li {
                    a href="/rant_add" { "Start Ranting!" }
                }
            }
        }
    }
}

