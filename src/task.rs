use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) done: bool,
    pub(crate) due: NaiveDateTime,
    pub(crate) tags: Vec<String>,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Task {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Task {
            name: row.get("name"),
            done: row.get("done"),
            due: NaiveDateTime::parse_from_str(row.get("due"), "%Y-%m-%d %H:%M:%S").unwrap(),
            tags: row.get::<'_, String, &str>("tags").split(",").map(|s| s.to_string()).collect(),
        })
    }
}
