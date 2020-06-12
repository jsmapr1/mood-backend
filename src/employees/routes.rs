use actix_web::{get, web, HttpResponse, Responder};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// This doesn't seem right. Figure out how to
// use local files
use crate::employees::model::Employee;
//use serde_json::json;

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Employee>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

#[get("/employees")]
async fn find_all() -> impl Responder {
    let u = read_user_from_file("./text.json").unwrap();
    HttpResponse::Ok().json(u)
}

#[get("/employees/{id}")]
async fn find() -> impl Responder {
    let u = read_user_from_file("./text.json").unwrap();
    HttpResponse::Ok().json(u.get(0))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
}
