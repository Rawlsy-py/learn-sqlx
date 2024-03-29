use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Worlds")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
