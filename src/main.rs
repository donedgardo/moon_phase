use actix_web::{App, get, HttpResponse, HttpServer, middleware, post, Responder};
use actix_files as fs;
use std::time::SystemTime;
use leptos::{view, IntoView, component};
use moon_phases::Moon;

#[component]
fn MoonAppMenu(cx: leptos::Scope) -> impl IntoView {
    let moon = Moon::new(SystemTime::now());
    let emoji = moon.phase_emoji();
    let phase = moon.phase_name();
    view! { cx,
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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting Http Server at 127.0.0.1:8000");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./static"))
            .service(index)
            .service(game)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}