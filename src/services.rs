use actix_web::{get, post, web, HttpResponse, Responder};

use crate::app_state;

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello, world");
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

#[get("/count")]
async fn counter(data: web::Data<app_state::AppState>) -> impl Responder {
    let mut count = data.counter.lock().unwrap();
    *count += 1;
    let output = format!("Counter: {}", count);
    return HttpResponse::Ok().body(output);
}

pub fn setup_services(config: &mut web::ServiceConfig) {
    config.service(hello).service(echo).service(counter);
}
