use actix_web::{get, HttpResponse, Responder};
use leptos::{component, IntoView};
use moon_phases::moon::Moon;
use std::time::SystemTime;

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

#[component]
fn MoonAppMenu(cx: leptos::Scope) -> impl IntoView {
    let moon = Moon::new(SystemTime::now());
    let emoji = moon.phase_emoji();
    let phase = moon.phase_name();
    leptos::view! { cx,
        <div
          id="app"
          class="mx-auto flex flex-col justify-center items-center space-y-4"
        >
          <p style="font-size: 180px">{emoji}</p>
          <h1 class="text-3xl font-bold underline text-white text-center">Lunar Harvest</h1>
          <h2 class="text-1xl font-bold text-white text-center">{phase}</h2>
          <button
            hx-get="/game"
            hx-target="#app"
            class="mx-auto px-4 py-1 text-1xl text-white font-semibold rounded-full border border-amber-300 hover:text-black hover:bg-amber-300 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-amber-300 focus:ring-offset-2"
          >
            Start Game
          </button>
        </div>
    }
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
    use actix_web::{test, App};
    use crate::router::index;

    #[actix_web::test]
    async fn test_index_status() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
