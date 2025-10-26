use crate::application::task::task_service::TaskService;
use actix_web::{HttpResponse, Responder, Scope, web};
use openapi_client::models::{NewTask, Task as TaskApiModel};

#[derive(Clone)]
pub struct TaskController {
    service: TaskService,
}

impl TaskController {
    pub fn new(service: TaskService) -> Self {
        Self { service }
    }

    pub fn scope(self) -> Scope {
        web::scope("/tasks")
            .app_data(web::Data::new(self.service))
            .route("", web::get().to(Self::list_tasks))
            .route("", web::post().to(Self::create_task))
            .route("/{id}", web::get().to(Self::get_task))
            .route("/{id}/done", web::put().to(Self::mark_done))
    }

    async fn list_tasks(service: web::Data<TaskService>) -> impl Responder {
        match service.list_tasks().await {
            Ok(tasks) => {
                let api_tasks: Vec<TaskApiModel> = tasks
                    .into_iter()
                    .map(|task| Into::<TaskApiModel>::into(task))
                    .collect();
                HttpResponse::Ok().json(api_tasks)
            }
            Err(e) => {
                eprintln!("Error listing tasks: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    async fn create_task(
        service: web::Data<TaskService>,
        payload: web::Json<NewTask>,
    ) -> impl Responder {
        match service.create_task(&payload.title).await {
            Ok(task) => HttpResponse::Created().json(Into::<TaskApiModel>::into(task)),
            Err(e) => {
                eprintln!("Error creating task: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    async fn get_task(path: web::Path<i64>, service: web::Data<TaskService>) -> impl Responder {
        let id = path.into_inner();
        match service.get_task(id).await {
            Ok(Some(task)) => HttpResponse::Ok().json(Into::<TaskApiModel>::into(task)),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => {
                eprintln!("Error fetching task: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    async fn mark_done(path: web::Path<i64>, service: web::Data<TaskService>) -> impl Responder {
        let id = path.into_inner();
        match service.mark_done(id).await {
            Ok(Some(task)) => HttpResponse::Ok().json(Into::<TaskApiModel>::into(task)),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => {
                eprintln!("Error marking task as done: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
