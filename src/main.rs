use actix_files::Files;
use actix_web::{App, HttpServer};
use sea_orm::{Database, Schema, ConnectionTrait};

mod infrastructure;
mod domain;
mod application;

use application::task::task_service::TaskService;
use infrastructure::persistence::task_repository::TaskRepository;
use infrastructure::api::task_controller::TaskController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Task Tracker API running at http://127.0.0.1:8080");
    println!("🚀 Swagger UI available at http://127.0.0.1:8080/swagger-ui/index.html");

    let db = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to embedded SQLite database");

    let schema = Schema::new(sea_orm::DatabaseBackend::Sqlite);

    let stmt = schema.create_table_from_entity(domain::task::Entity);
    db.execute(db.get_database_backend().build(&stmt))
        .await
        .expect("Failed to create schema");

    let repo = TaskRepository::new(db);
    let service = TaskService::new(repo);
    let controller = TaskController::new(service);

    HttpServer::new(move || {
        App::new()
            .service(controller.clone().scope())
            .service(Files::new("/openapi", "./openapi").index_file("openapi.yaml"))
            .service(Files::new("/swagger-ui", "./target/static/swagger-ui").index_file("task-ui.html"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
