use sqlx::{
    sqlite::{SqliteQueryResult, SqliteRow},
    Row,
};

pub async fn tag_task(
    pool: &sqlx::SqlitePool,
    id: u32,
    tag_id: u32,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO tagged (task_id, tag_id) VALUES (?, ?)")
        .bind(id)
        .bind(tag_id)
        .execute(pool)
        .await
}

pub async fn create_tag(pool: &sqlx::SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    sqlx::query("INSERT INTO tags (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await
        .map(|result| result.last_insert_rowid())
}

pub async fn fetch_tag_id_with_name(
    pool: &sqlx::SqlitePool,
    name: &str,
) -> Result<u32, sqlx::Error> {
    sqlx::query("SELECT id FROM tags WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await
        .map(|row: SqliteRow| row.get::<u32, &str>("id"))
}
