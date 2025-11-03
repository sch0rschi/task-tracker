use std::sync::Arc;
use actix_files::Files;
use actix_web::{App, HttpServer, middleware};
use actix_cors::Cors;
use sea_orm::{ConnectionTrait, Database, Schema};

pub mod api;
pub mod mapper;
pub mod persistence;

use application::task::task_service::TaskService;
use persistence::repository::task_repository::TaskRepository;
use api::task_controller::TaskController;
use application::task::task_repository_trait::TaskRepositoryTrait;
use application::task::task_service_trait::TaskServiceTrait;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Swagger UI available at http://127.0.0.1:8080/swagger-ui/index.html");
    println!("Task Tracker running at http://127.0.0.1:8080");

    let database_connection = Arc::new(Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database"));

    let schema = Schema::new(sea_orm::DatabaseBackend::Sqlite);
    let stmt = schema.create_table_from_entity(persistence::entity::task::Entity);
    database_connection.execute(database_connection.get_database_backend().build(&stmt))
        .await
        .expect("Failed to create schema");

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
