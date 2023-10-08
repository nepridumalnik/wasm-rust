use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
}

pub async fn process_signup(data: web::Form<Info>) -> impl Responder {
    if data.name.is_empty() {
        HttpResponse::BadRequest().body("User`s name is empty")
    } else {
        println!("{:?}", data);
        HttpResponse::Ok().body(format!("Successfully saved user: {}", data.name))
    }
}
