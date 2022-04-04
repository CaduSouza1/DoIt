use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(short, long, value_name = "CONFIG-FILE")]
    pub config: Option<PathBuf>,

    #[clap(short, long, value_name = "TASKS-FILE")]
    pub tasks_filepath: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new list
    NewList {
        #[clap(short, long)]
        name: String,
    },

    /// Adds a task to a list
    AddTask {
        #[clap(short, long)]
        list: String,

        #[clap(short, long)]
        title: String,

        #[clap(short, long)]
        description: String,

        #[clap(short, long)]
        completed: Option<bool>,
    },

    /// Removes a list and all of its items
    RemoveList {
        #[clap(short, long)]
        name: String,
    },

    /// Removes a task from a list
    RemoveTask {
        #[clap(short, long)]
        list_name: String,

        #[clap(short, long)]
        index: usize,
    },

    /// Marks a task as completed
    CompleteTask {
        #[clap(short, long)]
        list_name: String,

        #[clap(short, long)]
        index: usize,
    },

    /// Marks all tasks of a list as completed
    CompleteList {
        #[clap(short, long)]
        name: String,
    },

    /// Prints the tasks of a list to stdout
    ShowList {
        #[clap(short, long)]
        name: String,
    },

    /// Prints all lists to stdout
    ShowAll {},
}
