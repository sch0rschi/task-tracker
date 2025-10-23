use actix_web::{web, HttpResponse, Responder};
use openapi_client::models::Task;

pub async fn get_task(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();

    if id == 42 {
        HttpResponse::Ok().json(Task {
            id,
            title: "Finish Rust project".to_string(),
            done: false,
        })
    } else {
        HttpResponse::NotFound().finish()
    }
}
