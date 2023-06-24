use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use tokio::sync::Mutex;

use crate::{bll, dal};

pub async fn run() -> Result<(), std::io::Error> {
    let client = dal::crud_ops::connect_postgres()
        .await
        .expect("Failed to connect to data base.");
    let data = web::Data::new(Arc::new(Mutex::new(client)));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/tasks", web::get().to(bll::logic::get_tasks_handler))
            .route("/tasks", web::post().to(bll::logic::add_task_handler))
            .route(
                "/tasks/{id}",
                web::put().to(bll::logic::update_task_handler),
            )
            .route(
                "/tasks/{id}",
                web::delete().to(bll::logic::delete_task_handler),
            )
            .route("/tasks", web::delete().to(bll::logic::delete_all_tasks_handler))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
