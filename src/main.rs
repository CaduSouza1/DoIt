mod app;
mod cli;
mod errors;
mod tasks;

use app::create_app;
use std::{fs::File, io::Write};
use tasks::TaskLists;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "tasks.json";

    let app = create_app();

    let mut task_lists = TaskLists::from_json_file(&File::open(&filepath)?)?;

    match app.get_matches().subcommand() {
        ("add", Some(add_matches)) => cli::parse_add_command(add_matches, &mut task_lists),
        ("remove", Some(rm_matches)) => cli::parse_remove_command(rm_matches, &mut task_lists)?,
        ("show", Some(show_matches)) => cli::parse_show_command(show_matches, &task_lists),
        ("complete", Some(complete_matches)) => {
            cli::parse_complete_command(complete_matches, &mut task_lists)?
        }
        ("", None) => (),
        _ => unreachable!(),
    };
    
    File::create("tasks.json")?.write_all(serde_json::to_string_pretty(&task_lists)?.as_bytes())?;
    Ok(())
}
