mod app_state;
mod connection;
mod users;

use actix_web::{web, App, HttpServer};
use app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let factory = || {
        let state: AppState = app_state::AppState::new();

        App::new()
            .configure(users::setup_services)
            .app_data(web::Data::new(state))
    };

    let server = HttpServer::new(factory).bind(("127.0.0.1", 80))?;
    return server.run().await;
}
