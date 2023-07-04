use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let idempotency_key = uuid::Uuid::new_v4();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Publish newsletter</title>
                </head>
                <body>
                    {msg_html}
                    <form action="/admin/newsletters" method="post">
                        <label>Title
                            <input
                                type="text"
                                placeholder="Enter Title"
                                name="title"
                            >
                        </label>
                        <label>HTML content
                            <input
                                type="text"
                                placeholder="Enter HTML content"
                                name="html"
                            >
                        </label>
                        <label>Text content
                            <input
                                type="text"
                                placeholder="Enter text content"
                                name="text"
                            >
                        </label>
                        <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
                        <button type="submit">Publish</button>
                    </form>
                </body>
            </html>"#,
        ))
}
