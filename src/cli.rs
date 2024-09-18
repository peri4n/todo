use std::path::PathBuf;
use sqlx::types::chrono::NaiveDateTime;
use clap::{Parser, Subcommand};

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
        due: NaiveDateTime

    },

    /// Init database
    Init,

    /// Masks a task a done
    Done {
        /// lists test values
        #[arg(short, long)]
        name: String,
    },

    /// Lists all tasks
    List,
}

