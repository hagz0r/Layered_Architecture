use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
}

impl From<tokio_postgres::Row> for Task {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
            due_date: row.get("due_date"),
        }
    }
}
