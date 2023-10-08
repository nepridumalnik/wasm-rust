mod reg_post;

use actix_web::{middleware::Logger, web, App, HttpServer};

/// Создать обработчик статических файлов
/// # Примечание
/// Должно идти самым последним, чтобы не было коллизии маршрутизации
fn handle_static_files() -> actix_files::Files {
    return actix_files::Files::new("/", "./web/static").index_file("index.html");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let factory = || {
        App::new()
            .wrap(Logger::default())
            .route("/signup", web::post().to(reg_post::process_signup))
            .service(handle_static_files())
    };

    let server = HttpServer::new(factory).bind(("127.0.0.1", 8080))?;
    server.run().await
}
