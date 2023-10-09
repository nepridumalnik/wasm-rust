pub mod app_state;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let factory = || {
        let state = app_state::AppState::new();

        let app = App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state))
            .route(
                "/register_user",
                web::post().to(app_state::database::register_user),
            );
        app
    };

    let server = HttpServer::new(factory).bind(("127.0.0.1", 8081))?;
    server.run().await
}
