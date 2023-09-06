use actix_web::{post, HttpResponse, Responder};

#[post("/color-change")]
pub async fn post() -> impl Responder {
    let body = leptos::ssr::render_to_string(|cx| leptos::view! { cx,
      <a-box position="-1 0.5 -3" rotation="0 45 0" color="#F3F" hx-post="/color-change" hx-trigger="click" hx-swap="outerHTML"></a-box>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

