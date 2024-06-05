use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}, What a cruel world!", name))
}

#[actix_web::post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Snooooooooooooooop!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(2)
    .run()
    .await
}
