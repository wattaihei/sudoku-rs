use actix_web::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json("[2,3,4]")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/1", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
