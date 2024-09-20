pub mod cli;
pub mod task;

use anyhow::Result;
use clap::Parser;
use cli::*;
use task::*;
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // ensure the database is created
    Sqlite::create_database("sqlite:tasks.db").await?;

    // create a connection pool
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:tasks.db").await?;

    match &cli.command {
        Some(Commands::Add { name, due }) => {
            let result = sqlx::query("INSERT INTO tasks (name, due) VALUES (?, ?)")
                .bind(name)
                .bind(due.to_string())
                .execute(&pool)
                .await?;

            println!("Task with id {} added", result.last_insert_rowid());
        }
        Some(Commands::Init) => {
            sqlx::query_file!("src/schema.sql")
            .execute(&pool)
            .await?;
            println!("Done initializing database");
        }
        Some(Commands::List) => {
            sqlx::query_as("
                SELECT tasks.*, GROUP_CONCAT(tags.name) AS tags  
                FROM tasks LEFT JOIN tagged ON tasks.id = tagged.task_id LEFT JOIN tags ON tagged.tag_id = tags.id
                GROUP BY tasks.id, tasks.name, tasks.due, tasks.done")
                .fetch_all(&pool)
                .await?
                .iter()
                .for_each(|task: &Task| {
                    println!("Task {}: {} (done: {}, due: {}, tags: {})", task.id, task.name, task.done, task.due.format("%Y-%m-%dT%H:%M:%S"), task.tags.join(","));
                });
            println!("Listing all tasks");
        }
        Some(Commands::Done { name }) => {
            sqlx::query("UPDATE tasks SET done = 1 WHERE name = ?")
                .bind(name)
                .execute(&pool)
                .await?;
            println!("Finishing task: {}", name);
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
