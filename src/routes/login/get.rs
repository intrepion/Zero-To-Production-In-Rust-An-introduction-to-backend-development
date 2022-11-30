use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::{env, fmt::Write};

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let client_url = env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:8000".to_owned());
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Login</title>
    </head>
    <body>
        {error_html}
        <p>Client URL: <a href="{client_url}">{client_url}</a></p>
        <form action="/login" method="post">
            <label>Username
                <input
                    type="text"
                    placeholder="Enter Username"
                    name="username"
                >
            </label>
            <label>Password
                <input
                    type="password"
                    placeholder="Enter Password"
                    name="password"
                >
            </label>
            <button type="submit">Login</button>
        </form>
    </body>
</html>"#,
        ))
}
