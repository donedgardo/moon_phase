use actix_web::{get, HttpResponse, Responder};
use leptos::IntoView;
use moon_phases::moon::MoonAppMenu;

#[get("/")]
async fn index() -> impl Responder {
    let html = leptos::ssr::render_to_string(|cx| leptos::view! { cx,
        <html>
          <head>
            <script src="https://unpkg.com/htmx.org@1.9.5" integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" crossorigin="anonymous"></script>
            <link href="/static/output.css" rel="stylesheet" />
          </head>
          <body class="bg-white dark:bg-slate-800 flex items-center">
              <MoonAppMenu />
          </body>
        </html>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/game")]
async fn game() -> impl Responder {
    let html = leptos::ssr::render_to_string(|cx| leptos::view! { cx,
        <div>game</div>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[cfg(test)]
mod integration_tests {
    use super::index;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_index_status() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
