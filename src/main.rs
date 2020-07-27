//use std::env;
use actix_web::{App, HttpServer };
mod employees;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(employees::init_routes)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}

