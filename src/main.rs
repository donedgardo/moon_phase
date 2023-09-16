use actix_web::{App, HttpServer, middleware};
use actix_files as fs;
use crate::router::{home, game, box_change};

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
            .service(router::auth_get)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
