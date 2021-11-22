use std::num::ParseIntError;

use crate::tasks::{TaskItem, TaskList, TaskLists};

// to put them in a match I needed to give the same return type to all.
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
            task_lists.add_list(&get_list_name(list_matches).unwrap(), &TaskList::new(&[]));
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::app::create_app;

    #[test]
    fn list_name_parsing() {
        let expected_result = "List name test";
        let matches = create_app().get_matches_from(&["test", "-n", expected_result]);
        let actual_result = get_list_name(&matches).unwrap();

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn task_title_parsing() {
        let expected_result = "Task title test";
        let matches = create_app().get_matches_from(&["test", "-t", expected_result]);
        let actual_result = get_task_title(&matches).unwrap();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn task_description_parsing() {
        let expected_result = "Task description test";
        let matches = create_app().get_matches_from(&["test", "-d", expected_result]);
        let actual_result = get_task_description(&matches).unwrap();

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn test_parse_add_list_command() {
        let list_name = "test list name";
        let matches = create_app().get_matches_from(&["doit", "add", "list", &list_name]);
        let mut task_lists = TaskLists::new(&HashMap::default());
        parse_add_command(&matches, &mut task_lists);

        assert_ne!(task_lists.tasks_lists.get(list_name), None);
    }

    #[test]
    fn test_parse_add_task_command() {
        let list_name = "test list name";
        let task = TaskItem::new("test task title", "test task description", false);
        let matches = create_app().get_matches_from(&[
            "doit",
            "add",
            "task",
            "-t",
            &task.title,
            "-d",
            &task.description,
        ]);
        let mut map = HashMap::with_capacity(1);
        map.insert(list_name.to_string(), TaskList::new(&[]));
        let mut task_lists = TaskLists::new(&map);
        parse_add_command(&matches, &mut task_lists);
    }

    #[test]
    fn test_parse_remove_command() {
        let matches = create_app().get_matches_from(&["doit"]);
    }

    #[test]
    fn test_parse_complete_command() {
        let matches = create_app().get_matches_from(&["doit"]);
    }
}
