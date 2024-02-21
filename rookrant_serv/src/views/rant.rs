use maud::{DOCTYPE, html, Markup};

pub fn add() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Rook Rant" }
        }
        body {
            h1 { "Rant About Something" }
            form method="POST" action="/rant_add" {
                textarea name="rant" rows="10" cols="80" {
                }
                br {}
                input type="submit" value="Add" { }
            }
        }
    }
}

