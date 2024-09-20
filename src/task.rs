use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

#[derive(Debug)]
pub struct Task {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) done: bool,
    pub(crate) due: NaiveDateTime,
    pub(crate) tags: Vec<String>,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Task {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Task {
            id: row.get("id"),
            name: row.get("name"),
            done: row.get("done"),
            due: NaiveDateTime::parse_from_str(row.get("due"), "%Y-%m-%d %H:%M:%S").unwrap(),
            tags: row
                .get::<'_, String, &str>("tags")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        })
    }
}

pub async fn fetch_tasks(pool: &sqlx::SqlitePool) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as("
        SELECT tasks.*, GROUP_CONCAT(tags.name) AS tags  
        FROM tasks LEFT JOIN tagged ON tasks.id = tagged.task_id LEFT JOIN tags ON tagged.tag_id = tags.id
        GROUP BY tasks.id, tasks.name, tasks.due, tasks.done")
        .fetch_all(pool)
        .await
}

pub async fn fetch_task_with_id(
    pool: &sqlx::SqlitePool,
    id: u32,
) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create_task(
    pool: &sqlx::SqlitePool,
    name: &str,
    due: NaiveDateTime,
) -> Result<i64, sqlx::Error> {
    sqlx::query("INSERT INTO tasks (name, due) VALUES (?, ?)")
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
    sqlx::query("UPDATE tasks SET done = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
}
