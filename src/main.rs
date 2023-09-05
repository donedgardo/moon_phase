use actix_web::{App, HttpServer, middleware};
use actix_files as fs;
use crate::router::{game, index};
mod router;

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
