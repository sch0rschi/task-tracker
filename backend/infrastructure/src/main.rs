use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use sea_orm::{Database};
use std::sync::Arc;
use sea_orm_migration::prelude::*;

use api::task_controller::TaskController;
use application::task::task_repository_trait::TaskRepositoryTrait;
use application::task::task_service::TaskService;
use application::task::task_service_trait::TaskServiceTrait;
use persistence::repository::task_repository::TaskRepository;
use crate::persistence::migration::lib::Migrator;

pub mod api;
pub mod mapper;
pub mod persistence;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Swagger UI available at http://127.0.0.1:8080/swagger-ui/index.html");
    println!("Task Tracker running at http://127.0.0.1:8080");

    let database_connection = Arc::new(Database::connect("postgres://admin:secret@localhost:5432/taskdb")
        .await
        .expect("Failed to connect to database"));

    Migrator::up(&*database_connection, None)
        .await
        .expect("Failed to run migrations");

    let task_repository: Arc<dyn TaskRepositoryTrait> = Arc::new(TaskRepository::new(database_connection));
    let task_service: Arc<dyn TaskServiceTrait> = Arc::new(TaskService::new(task_repository));
    let task_controller: Arc<TaskController> = Arc::new(TaskController::new(task_service));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(task_controller.configure())
            .service(Files::new("/openapi", "../../openapi").index_file("openapi.yaml"))
            .service(Files::new("/swagger-ui", "../../target/static/swagger-ui").index_file("index.html"))
            .service(
                Files::new("/", "../../target/static/frontend")
                    .index_file("index.html")
                    .prefer_utf8(true)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
