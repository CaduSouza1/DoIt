mod app;
mod cli;
mod tasks;

use app::create_app;
use std::path::Path;
use std::{fs::File, io::Write};
use tasks::TaskLists;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();
    let matches = app.get_matches();
    let default_file = Path::new("tasks.json"); // default file for read and write operations
    let mut task_lists = TaskLists::from_json_file(&File::open(&default_file)?)?;

    match matches.subcommand() {
        ("add", Some(add_matches)) => cli::parse_add_command(add_matches, &mut task_lists),
        ("remove", Some(rm_matches)) => cli::parse_remove_command(rm_matches, &mut task_lists)?,
        ("show", Some(show_matches)) => cli::parse_show_command(show_matches, &task_lists),
        ("complete", Some(complete_matches)) => {
            cli::parse_complete_command(complete_matches, &mut task_lists)?
        }
        ("", None) => (),
        _ => unreachable!(),
    };

    let save_file = matches
        .value_of("file")
        .or(Some(default_file.to_str().unwrap()))
        .unwrap();

    File::create(save_file)?.write_all(serde_json::to_string_pretty(&task_lists)?.as_bytes())?;
    Ok(())
}
