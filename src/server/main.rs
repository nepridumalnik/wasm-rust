use actix_web::{App, HttpServer};

/// Создать обработчик статических файлов
fn handle_static_files() -> actix_files::Files {
    return actix_files::Files::new("/", "./web/static").index_file("index.html");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(handle_static_files()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
