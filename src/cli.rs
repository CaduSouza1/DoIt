use crate::tasks::{TaskItem, TaskList, TaskLists};
use std::path::Path;
use std::{fs::File, num::ParseIntError};

pub fn parse_add_command(matches: &clap::ArgMatches, task_lists: &mut TaskLists) {
    // Most of the values used in this function have a default value or throw an error in the clap argument parsing stage.
    // So it isn't a problem to just call .unwrap()
    match matches.subcommand() {
        ("task", Some(task_matches)) => {
            task_lists
                .add_task(
                    &get_list_name(task_matches).unwrap(),
                    &TaskItem::new(
                        &get_task_title(task_matches).unwrap(),
                        &get_task_description(task_matches).unwrap(),
                        false,
                    ),
                )
                .expect("No list with given name found");
        }
        ("list", Some(list_matches)) => {
            let list_name = get_list_name(list_matches).unwrap();
            task_lists.add_list(&list_name, &TaskList::new(&list_name, &[]));
        }
        ("", None) => {}
        _ => unreachable!(),
    }
}

pub fn parse_remove_command(
    matches: &clap::ArgMatches,
    task_lists: &mut TaskLists,
) -> Result<(), ParseIntError> {
    match matches.subcommand() {
        ("task", Some(task_matches)) => {
            let list_name = get_list_name(task_matches).unwrap();
            let index = get_index(task_matches).unwrap()?;

            task_lists.remove_task(&list_name, index);
        }
        ("list", Some(list_matches)) => {
            task_lists.remove_list(&get_list_name(list_matches).unwrap());
        }
        ("", None) => {}
        _ => unreachable!(),
    }

    Ok(())
}

pub fn parse_show_command(matches: &clap::ArgMatches, task_lists: &TaskLists) {
    match matches.subcommand() {
        ("list", Some(list_matches)) => {
            println!(
                "{}",
                task_lists
                    .format_task_list(&get_list_name(list_matches).unwrap(), 1)
                    .unwrap()
            );
        }
        ("all", _) => {
            println!("{}", task_lists.format_all_tasks(1));
        }
        ("", None) => {}
        _ => unreachable!(),
    }
}

pub fn parse_complete_command(
    matches: &clap::ArgMatches,
    task_lists: &mut TaskLists,
) -> Result<(), ParseIntError> {
    match matches.subcommand() {
        ("task", Some(task_matches)) => {
            let list_name = get_list_name(task_matches).unwrap();
            let index = get_index(task_matches).unwrap()?;

            task_lists.complete_task(&list_name, index);
        }
        ("list", Some(list_matches)) => {
            task_lists.complete_list(&get_list_name(list_matches).unwrap());
        }
        ("", None) => {}
        _ => unreachable!(),
    }

    Ok(())
}

pub fn get_list_name(matches: &clap::ArgMatches) -> Option<String> {
    Some(matches.value_of("list")?.to_string())
}

pub fn get_task_title(matches: &clap::ArgMatches) -> Option<String> {
    Some(matches.value_of("title")?.to_string())
}

pub fn get_task_description(matches: &clap::ArgMatches) -> Option<String> {
    Some(matches.value_of("description")?.to_string())
}

pub fn get_index(matches: &clap::ArgMatches) -> Option<Result<usize, ParseIntError>> {
    Some(matches.value_of("index")?.parse())
}

pub fn get_read_file(matches: &clap::ArgMatches) -> Option<Result<File, std::io::Error>> {
    matches
        .value_of("read-file")
        .or(matches.value_of("file"))
        .and_then(|filepath| Some(File::open(Path::new(filepath))))
}

pub fn get_save_file(matches: &clap::ArgMatches) -> Option<Result<File, std::io::Error>> {
    matches
        .value_of("save-file")
        .or(matches.value_of("file"))
        .or(Some(&format!(
            "{}/tasks.json",
            home::home_dir().unwrap().to_str().unwrap()
        )))
        .and_then(|filepath| Some(File::create(Path::new(filepath))))
}
