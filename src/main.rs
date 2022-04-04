mod app;
mod tasks;

use std::{fs::File, io::Write};

use app::{Cli, Commands};
use clap::Parser;
use tasks::TaskLists;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let tasks_filepath = cli.tasks_filepath.or(Some("tasks.json".into())).unwrap();
    let mut lists = TaskLists::from_json_file(&File::open(&tasks_filepath)?)?;

    match cli.command {
        Commands::NewList { name } => {
            lists
                .add_list(&name, &tasks::TaskList::new(&name, &[]))
                .unwrap();
        }

        Commands::AddTask {
            list,
            title,
            description,
            completed,
        } => {
            lists
                .add_task(
                    &list,
                    &tasks::TaskItem::new(&title, &description, completed.or(Some(false)).unwrap()),
                )
                .unwrap();
        }

        Commands::RemoveList { name } => {
            lists.remove_list(&name).unwrap();
        }

        Commands::RemoveTask { list_name, index } => {
            lists.remove_task(&list_name, index).unwrap();
        }

        Commands::CompleteList { name } => {
            lists.complete_list(&name).unwrap();
        }

        Commands::CompleteTask { list_name, index } => {
            lists.complete_task(&list_name, index).unwrap();
        }

        Commands::ShowList { name } => {
            println!("{}", lists.format_task_list(&name, 1).unwrap())
        }

        Commands::ShowAll {} => {
            println!("{}", lists.format_all_tasks(1))
        }
    }

    File::create(tasks_filepath)?.write_all(serde_json::to_string_pretty(&lists)?.as_bytes())?;
    Ok(())
}
