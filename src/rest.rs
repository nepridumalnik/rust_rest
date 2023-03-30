use std::{sync::Mutex, task::Poll};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    counter: Mutex<u32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello, world");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

#[get("/count")]
async fn counter(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.counter.lock().unwrap();
    *count += 1;
    let output = format!("Counter: {}", count);
    return HttpResponse::Ok().body(output);
}

pub struct RestServer {}

impl RestServer {
    pub fn new() -> RestServer {
        RestServer {}
    }

    pub async fn run(&self) {
        let factory = || {
            App::new()
                .service(hello)
                .service(echo)
                .service(counter)
                .app_data(web::Data::new(AppState {
                    counter: Mutex::new(0),
                }))
        };

        let server = HttpServer::new(factory).bind(("127.0.0.1", 80));
        server.unwrap().run().await;
    }
}
