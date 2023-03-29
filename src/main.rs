use actix_web::{web, App, HttpServer};

mod app_state;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let factory = || {
        App::new()
            .configure(services::setup_services)
            .app_data(web::Data::new(app_state::AppState::new()))
    };

    let server = HttpServer::new(factory).bind(("127.0.0.1", 80))?;
    return server.run().await;
}
