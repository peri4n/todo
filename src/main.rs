pub mod cli;

use anyhow::Result;
use clap::Parser;
use sqlx::migrate::MigrateDatabase;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Sqlite, Row};
use cli::*;


#[derive(Debug)]
struct Task {
    name: String,
    done: bool,
    due: NaiveDateTime,
    tags: Vec<String>,
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // ensure the database is created
    Sqlite::create_database("sqlite:tasks.db").await?;

    // create a connection pool
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:tasks.db").await?;

    match &cli.command {
        Some(Commands::Add { name , due}) => {
            sqlx::query("INSERT INTO tasks (name, due) VALUES (?, ?)")
                .bind(name)
                .bind(due.to_string())
                .execute(&pool)
                .await?;
        },
        Some(Commands::Init) => {
            sqlx::query("CREATE TABLE tasks (
                    id INTEGER PRIMARY KEY, 
                    name TEXT NOT NULL, 
                    done BOOLEAN NOT NULL DEFAULT 0,
                    due DATE);")
                .execute(&pool)
                .await?;

            sqlx::query("CREATE TABLE tags (
                        id INTEGER PRIMARY KEY, 
                        name TEXT NOT NULL
                    );")
                .execute(&pool)
                .await?;

            sqlx::query("CREATE TABLE tagged (
                task_id INTEGER,
                tag_id INTEGER
            );").execute(&pool).await?;
            println!("Done initializing database");
        },
        Some(Commands::List) => {
            sqlx::query_as::<_, Task>(r#"
                SELECT tasks.*, GROUP_CONCAT(tags.name) AS tags  
                FROM tasks LEFT JOIN tagged ON tasks.id = tagged.task_id LEFT JOIN tags ON tagged.tag_id = tags.id
                GROUP BY tasks.id, tasks.name, tasks.due, tasks.done"#)
                .fetch_all(&pool)
                .await?
                .iter()
                .for_each(|task| {
                    println!("Task: {} (done: {}, due: {}, tags: {})", task.name, task.done, task.due.format("%Y-%m-%dT%H:%M:%S"), task.tags.join(", "));
                });
            println!("Listing all tasks");
        },
        Some(Commands::Done { name }) => {
            sqlx::query("UPDATE tasks SET done = 1 WHERE name = ?")
                .bind(name)
                .execute(&pool)
                .await?;
            println!("Finishing task: {}", name);
        },
        None => {
            println!("No command provided");
        },
    }

    Ok(())
}
