use crate::environment;
use tokio_postgres::{Client, Error, NoTls};

use super::models::task::Task;

pub async fn add_task(client: &Client, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let statement = client
        .prepare(
            "INSERT INTO tasks (title, description, completed, due_date) VALUES ($1, $2, $3, $4)",
        )
        .await?;
    client
        .execute(
            &statement,
            &[
                &task.title,
                &task.description,
                &task.completed,
                &task.due_date,
            ],
        )
        .await?;
    Ok(())
}

pub async fn get_tasks(client: &Client) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let statement = client.prepare("SELECT * FROM tasks").await?;
    let rows = client.query(&statement, &[]).await?;
    let tasks = rows.into_iter().map(Task::from).collect();
    Ok(tasks)
}

pub async fn update_task(client: &Client, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let statement = client
        .prepare("UPDATE tasks SET title = $1, description = $2, completed = $3, due_date = $4 WHERE id = $5")
        .await?;
    client
        .execute(
            &statement,
            &[
                &task.title,
                &task.description,
                &task.completed,
                &task.due_date,
                &task.id,
            ],
        )
        .await?;
    Ok(())
}

pub async fn delete_task(client: &Client, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let statement = client.prepare("DELETE FROM tasks WHERE id = $1").await?;
    client.execute(&statement, &[&id]).await?;
    Ok(())
}
pub async fn delete_all_tasks(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    client.batch_execute("DELETE FROM tasks; SELECT setval('tasks_id_seq', 1, false)").await?;
    Ok(())
}


pub async fn connect_postgres() -> Result<Client, Error> {
    let cfg = environment::Config::new();
    let (client, connection) = tokio_postgres::connect(&cfg.connection_string, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
