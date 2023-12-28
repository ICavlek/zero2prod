use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn subscribe_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Subscribe to newsletter</title>
</head>
<body>
    {msg_html}
    <form action="/subscriptions" method="post">
        <label>Name
            <input type="Name" placeholder="Enter your name" name="name">
        </label>
        <br>
        <label>E-Mail
            <input type="email" placeholder="Enter new email" name="email">
        </label>
        <br>
        <button type="submit">Subscribe</button>
    </form>
    <p><a href="/">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
