use crate::mapper::task_mapper::ToApiModel;
use actix_web::{HttpResponse, Responder, Scope, web};
use application::task::task_service_trait::TaskServiceTrait;
use openapi_client::models::{NewTask, RenameTask, Task as TaskApiModel};
use std::sync::Arc;

#[derive(Clone)]
pub struct TaskController {
    task_service: Arc<dyn TaskServiceTrait>,
}

impl TaskController {
    pub fn new(task_service: Arc<dyn TaskServiceTrait>) -> Self {
        Self { task_service }
    }

    pub fn configure(&self) -> Scope {
        web::scope("/tasks")
            .app_data(web::Data::new(self.task_service.clone()))
            .route("", web::get().to(Self::list_tasks))
            .route("", web::post().to(Self::create_task))
            .route("/{id}", web::get().to(Self::get_task))
            .route("/{id}/done", web::put().to(Self::mark_done))
            .route("/{id}/title", web::put().to(Self::rename_task))
    }

    async fn list_tasks(service: web::Data<Arc<dyn TaskServiceTrait>>) -> impl Responder {
        match service.list_tasks().await {
            Ok(tasks) => {
                let api_tasks: Vec<TaskApiModel> =
                    tasks.into_iter().map(ToApiModel::to_api_model).collect();
                HttpResponse::Ok().json(api_tasks)
            }
            Err(e) => {
                eprintln!("Error listing tasks: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    async fn create_task(
        service: web::Data<Arc<dyn TaskServiceTrait>>,
        payload: web::Json<NewTask>,
    ) -> impl Responder {
        match service.create_task(&payload.title).await {
            Ok(task) => HttpResponse::Created().json(ToApiModel::to_api_model(task)),
            Err(e) => {
                eprintln!("Error creating task: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    async fn get_task(
        path: web::Path<i64>,
        service: web::Data<Arc<dyn TaskServiceTrait>>,
    ) -> impl Responder {
        let id = path.into_inner();
        match service.get_task(id).await {
            Ok(Some(task)) => HttpResponse::Ok().json(ToApiModel::to_api_model(task)),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => {
                eprintln!("Error fetching task: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    async fn mark_done(
        path: web::Path<i64>,
        service: web::Data<Arc<dyn TaskServiceTrait>>,
    ) -> impl Responder {
        let id = path.into_inner();
        match service.mark_done(id).await {
            Ok(Some(task)) => HttpResponse::Ok().json(ToApiModel::to_api_model(task)),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => {
                eprintln!("Error marking task as done: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub async fn rename_task(
        path: web::Path<i64>,
        new_title: web::Json<RenameTask>,
        service: web::Data<Arc<dyn TaskServiceTrait>>,
    ) -> impl Responder {
        let id = path.into_inner();
        let new_title = new_title.into_inner().title;

        match service.rename_task(id, new_title).await {
            Ok(Some(task)) => HttpResponse::Ok().json(ToApiModel::to_api_model(task)),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => {
                eprintln!("Error renaming task: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
