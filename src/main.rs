mod app;
mod cli;
mod tasks;

use app::create_app;
use std::collections::HashMap;
use std::io::Write;
use tasks::TaskLists;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();
    let matches = app.get_matches();
    let mut task_lists = if let Some(read_file) = cli::get_read_file(&matches) {
        TaskLists::from_json_file(&read_file?)?
    } else {
        TaskLists::new(&HashMap::new())
    };

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

    let mut save_file = cli::get_save_file(&matches).unwrap()?;
    save_file.write_all(serde_json::to_string_pretty(&task_lists)?.as_bytes())?;
    Ok(())
}
