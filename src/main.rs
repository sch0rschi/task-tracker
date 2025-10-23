use actix_files::Files;
use actix_web::{App, HttpServer, web};

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Task Tracker API running at http://127.0.0.1:8080");
    println!("🚀 Task Tracker swagger running at http://127.0.0.1:8080/swagger-ui/index.html");

    HttpServer::new(|| {
        App::new()
            .route("/tasks/{id}", web::get().to(controller::get_task))
            .service(Files::new("/openapi", "./openapi").index_file("openapi.yaml"))
            .service(Files::new("/swagger-ui", "./target/static/swagger-ui").index_file("task-ui.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
