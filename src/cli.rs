use clap::{Parser, Subcommand};
use sqlx::types::chrono::NaiveDateTime;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    pub(super) command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Name of the task
        #[arg(short, long)]
        name: String,

        #[arg(short, long, value_parser(|v: &str| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S")))]
        due: NaiveDateTime,
    },

    /// Init database
    Init,

    /// Masks a task a done
    Done {
        /// Id of the task
        #[arg(short, long)]
        id: u32,
    },

    /// Lists all tasks
    List,

    /// Tag a task
    Tag {
        /// Id of the task
        #[arg(short, long)]
        id: u32,

        /// Name of the tag
        #[arg(short, long)]
        tag: String,
    },
}
