use actix_web::{get, post, App, HttpServer, HttpResponse, Responder};
use dotenv;

#[get("/get")]
fn get() -> impl Responder {
  HttpResponse::Ok().body("GET 成功")
}

#[post("/post")]
fn post() -> impl Responder {
  HttpResponse::Ok().body("POST 成功")
}

fn main() -> std::io::Result<()> {
  HttpServer::new(
    || App::new ()
        .service(get)
        .service(post)
    )
    .bind(dotenv::var("HOST").unwrap() + ":" + &dotenv::var("PORT").unwrap())?
    .run()
}