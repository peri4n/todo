pub mod cli;
pub mod tag;
pub mod task;

use anyhow::Result;
use clap::Parser;
use cli::*;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use tag::*;
use task::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // ensure the database is created
    Sqlite::create_database("sqlite:tasks.db").await?;

    // create a connection pool
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:tasks.db").await?;

    match &cli.command {
        Some(Commands::Add { name, due }) => {
            let task_id = create_task(&pool, name, *due).await?;
            println!("Task with id {} added", task_id);
        }
        Some(Commands::Init) => {
            sqlx::query_file!("src/schema.sql").execute(&pool).await?;
            println!("Done initializing database");
        }
        Some(Commands::List) => {
            fetch_tasks(&pool).await?.iter().for_each(|task: &Task| {
                println!("{}", task);
            });
        }
        Some(Commands::Done { id }) => {
            finish_task(&pool, *id).await?;
            println!("Finished task {}", id);
        }
        Some(Commands::Tag { id, tag }) => {
            // check if task exists
            if fetch_task_with_id(&pool, *id).await?.is_some() {
                // check if the tag exists
                if let Ok(tag_id) = fetch_tag_id_with_name(&pool, tag).await {
                    tag_task(&pool, *id, tag_id).await?;
                } else {
                    // if not present create the tag
                    let tag_id = create_tag(&pool, tag).await?;
                    tag_task(&pool, *id, tag_id as u32).await?;
                }
            } else {
                // task not present
                return Err(anyhow::anyhow!("Task with id {} not found", id));
            }
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
