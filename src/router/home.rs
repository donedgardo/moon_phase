use actix_web::{get, HttpResponse, Responder};
use leptos::{IntoView, view};
use moon_phases::moon::component::MoonAppMenu;

#[get("/")]
pub async fn get() -> impl Responder {
    let html = leptos::ssr::render_to_string(|cx| view! { cx,
        <html>
          <head>
            <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous"></script>
            <script src="https://aframe.io/releases/1.4.0/aframe.min.js"></script>
            <script src="https://unpkg.com/aframe-environment-component@1.3.2/dist/aframe-environment-component.min.js"></script>
            <link href="/static/output.css" rel="stylesheet" />
          </head>
          <body class="bg-white dark:bg-slate-800 flex items-center" hx-boost="true">
              <MoonAppMenu />
          </body>
        </html>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_index_status() {
        let mut app = test::init_service(App::new().service(get)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
