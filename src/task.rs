use std::fmt::Display;

use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Cell, ContentArrangement, Table};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

#[derive(Debug)]
pub struct Task {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) finished: Option<NaiveDateTime>,
    pub(crate) due: NaiveDateTime,
    pub(crate) tags: Vec<String>,
}

pub fn print_task_table(tasks: Vec<Task>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(120)
        .set_header(vec!["ID", "Name", "Due", "Finished", "Tags"]);

    tasks.iter().for_each(|task: &Task| {
        table.add_row(vec![
            Cell::new(task.id.to_string()),
            Cell::new(task.name.clone()),
            Cell::new(task.due.format("%Y-%m-%d %H:%M:%S").to_string()),
            Cell::new(
                task.finished
                    .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or("".to_string()),
            ),
            Cell::new(task.tags.join(",")),
        ]);
    });

    println!("{}", table);
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Task {}: {} (due: {}, finished: {}, tags: {})",
            self.id,
            self.name,
            self.due.format("%Y-%m-%d %H:%M:%S"),
            self.finished
                .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or("".to_string()),
            self.tags.join(",")
        ))
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Task {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Task {
            id: row.get("id"),
            name: row.get("name"),
            finished: NaiveDateTime::parse_from_str(row.get("finished_date"), "%Y-%m-%d %H:%M:%S")
                .ok(),
            due: NaiveDateTime::parse_from_str(row.get("due_date"), "%Y-%m-%d %H:%M:%S").unwrap(),
            tags: row
                .get::<'_, String, &str>("tags")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        })
    }
}

pub async fn fetch_tasks(pool: &sqlx::SqlitePool) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as(
        "
        SELECT 
            tasks.*, GROUP_CONCAT(tags.name) AS tags  
        FROM tasks 
            LEFT JOIN tagged ON tasks.id = tagged.task_id 
            LEFT JOIN tags ON tagged.tag_id = tags.id
        GROUP BY 
            tasks.id, tasks.name, tasks.due_date, tasks.finished_date
        ORDER BY 
            tasks.due_date",
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_task_with_id(
    pool: &sqlx::SqlitePool,
    id: u32,
) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as(
        "
        SELECT 
            tasks.*, GROUP_CONCAT(tags.name) AS tags  
        FROM tasks 
            LEFT JOIN tagged ON tasks.id = tagged.task_id 
            LEFT JOIN tags ON tagged.tag_id = tags.id
        WHERE 
            tasks.id = ?
        GROUP BY 
            tasks.id, tasks.name, tasks.due_date, tasks.finished_date
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_task(
    pool: &sqlx::SqlitePool,
    name: &str,
    due: NaiveDateTime,
) -> Result<i64, sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO 
            tasks (name, due_date) 
        VALUES 
            (?, ?)",
    )
    .bind(name)
    .bind(due)
    .execute(pool)
    .await
    .map(|result| result.last_insert_rowid())
}

pub async fn finish_task(
    pool: &sqlx::SqlitePool,
    id: u32,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "
        UPDATE 
            tasks 
        SET 
            finished_date = CURRENT_TIMESTAMP 
        WHERE 
            id = ?",
    )
    .bind(id)
    .execute(pool)
    .await
}
