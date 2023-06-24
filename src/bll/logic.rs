use std::sync::Arc;
use tokio::sync::Mutex;

use crate::crud_ops::{add_task, delete_task, get_tasks, update_task};
use crate::dal::crud_ops::delete_all_tasks;
use crate::dal::models::task::Task;
use actix_web::{web, HttpResponse, Responder};
use tokio_postgres::Client;

pub async fn get_tasks_handler(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    println!("GET tasks handled.");
    let client = client.lock().await;
    match get_tasks(&client).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_task_handler(
    client: web::Data<Arc<Mutex<Client>>>,
    task: web::Json<Task>,
) -> impl Responder {
    let client = client.lock().await;
    match add_task(&client, &task.0).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_task_handler(
    client: web::Data<Arc<Mutex<Client>>>,
    id: web::Path<i32>,
) -> HttpResponse {
    let client = client.lock().await;
    match delete_task(&client, id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_task_handler(
    client: web::Data<Arc<Mutex<Client>>>,
    task: web::Json<Task>,
) -> impl Responder {
    let client = client.lock().await;
    match update_task(&client, &task.0).await {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn delete_all_tasks_handler(
    client: web::Data<Arc<Mutex<Client>>>
) -> impl Responder {
    let client = client.lock().await;
    match delete_all_tasks(&client).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
