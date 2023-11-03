use crate::router::{auth, box_change, chat as chat_router, game, home, ws};
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};

mod chat;
mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting Http Server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./static"))
            .service(home::get)
            .service(game::get_game)
            .service(game::get_moon_stats)
            .service(box_change::post)
            .service(auth::auth_get)
            .service(chat_router::chat_get)
            .service(web::resource("/ws/").route(web::get().to(ws::ws_index)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
