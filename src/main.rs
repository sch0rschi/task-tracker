use actix_web::{web, App, HttpServer};

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Task Tracker API running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/tasks/{id}", web::get().to(controller::get_task))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
