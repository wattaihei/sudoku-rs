use actix_web::*;
use serde_json::json;
use serde::*;
use sudoku::sudoku_make::make;
use sudoku::sudoku_solve::solve;

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(description))
            .route("/problems/{level}", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
