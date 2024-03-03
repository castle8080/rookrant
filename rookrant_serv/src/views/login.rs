use maud::{html, Markup};
use crate::views::main_layout::main_layout;
use crate::services::user_repository::User;

pub fn login(user: Option<User>) -> Markup {
    main_layout(user, html!(
        h1 { "Login" }
        form method="POST" action="/auth/login_start" {
            input type="submit" value="Login" { }
        }
    ))
}

pub fn login_complete(user: Option<User>) -> Markup {
    main_layout(user, html!(
        h2 { "Login Complete" }
        script { "
            document.location = '/';
        " }
    ))
}