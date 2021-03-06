use actix_web::*;
use actix_cors::Cors;
use serde_json::json;
use serde::*;
use std::env;
mod core;
use crate::core::sudoku_make::make;
use crate::core::sudoku_solve::solve;

#[derive(Deserialize)]
struct RequestParam {
    level : i32
}


async fn description() -> impl Responder {
    HttpResponse::Ok().body("please specify problem level")
}

async fn index(path : web::Path<RequestParam>) -> impl Responder {
    if path.level > 5 {
        return HttpResponse::BadRequest().body("level must be no more than 5");
    } else if path.level < 1 {
        return HttpResponse::BadRequest().body("level must be no less than 1");
    }
    let problem = make(path.level);
    let answer = solve(problem, path.level);
    let ret_json = json!({
        "problem" : problem.to_vec(),
        "answer" : answer.2.to_vec()
    });
    HttpResponse::Ok().json(ret_json.to_string())
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                .allowed_origin("http://localhost:8080")
                .allowed_origin("https://sudoku-front.herokuapp.com")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish()
            )
            .service(web::resource("/")
                .route(web::get().to(description))
            )
            .service(web::resource("/problems/{level}")
                .route(web::get().to(index))
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
    Ok(())
}
