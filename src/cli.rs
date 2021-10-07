use std::num::ParseIntError;

use crate::tasks;

pub fn parse_add_command(matches: &clap::ArgMatches, program: &mut tasks::Program) {
    match matches.subcommand() {
        ("task", Some(task_matches)) => {
            program
                .add_task(
                    &get_list_name(task_matches)
                        .or(Some("default".to_string()))
                        .unwrap(),
                    &tasks::TaskItem::new(
                        &get_task_title(task_matches).expect("No title given"),
                        &get_task_description(task_matches)
                            .or(Some("".to_string()))
                            .unwrap(),
                        false,
                    ),
                )
                .expect("No list with given name found");
        }
        ("list", Some(list_matches)) => {
            program.add_list(
                &get_list_name(list_matches).expect("No list name given"),
                &tasks::TaskList::new(&[]),
            );
        }
        ("", None) => {}
        _ => unreachable!(),
    }
}

pub fn parse_remove_command(matches: &clap::ArgMatches, program: &mut tasks::Program) {
    match matches.subcommand() {
        ("task", Some(task_matches)) => {
            let list_name = get_list_name(task_matches)
                .or(Some("default".to_string()))
                .unwrap();
            let index = get_index(task_matches).unwrap().unwrap();

            program.remove_task(&list_name, index);
        }
        ("list", Some(list_matches)) => {
            program.remove_list(&get_list_name(list_matches).expect("No list name given"));
        }
        ("", None) => {}
        _ => unreachable!(),
    }
}

pub fn parse_show_command(matches: &clap::ArgMatches, program: &tasks::Program) {
    match matches.subcommand() {
        ("list", Some(list_matches)) => {
            println!(
                "{}",
                program
                    .format_task_list(&get_list_name(list_matches).expect("No list name given"), 1)
                    .unwrap()
            );
        }
        ("all", Some(_)) => {
            println!("{}", program.format_all_tasks(1));
        }
        ("", None) => {}
        _ => unreachable!(),
    }
}

pub fn parse_complete_command(matches: &clap::ArgMatches, program: &mut tasks::Program) {
    match matches.subcommand() {
        ("task", Some(task_matches)) => {
            let list_name = get_list_name(task_matches)
                .or(Some("default".to_string()))
                .unwrap();
            let index = get_index(task_matches).unwrap().unwrap();

            program.complete_task(&list_name, index);
        }
        ("list", Some(list_matches)) => {
            program.complete_list(&get_list_name(list_matches).expect("No list name given"));
        }
        ("", None) => {}
        _ => unreachable!(),
    }
}

pub fn get_list_name(matches: &clap::ArgMatches) -> Option<String> {
    matches
        .value_of("name")
        .and_then(|name| Some(name.to_string()))
}

pub fn get_task_title(matches: &clap::ArgMatches) -> Option<String> {
    matches
        .value_of("title")
        .and_then(|title| Some(title.to_string()))
}

pub fn get_task_description(matches: &clap::ArgMatches) -> Option<String> {
    matches
        .value_of("description")
        .and_then(|description| Some(description.to_string()))
}

pub fn get_index(matches: &clap::ArgMatches) -> Option<Result<usize, ParseIntError>> {
    matches
        .value_of("index")
        .and_then(|index| Some(index.parse()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_app() -> clap::App<'static, 'static> {
        clap::App::new("test").args(&[
            clap::Arg::with_name("name").takes_value(true).short("n"),
            clap::Arg::with_name("title").takes_value(true).short("t"),
            clap::Arg::with_name("description")
                .takes_value(true)
                .short("d"),
        ])
    }

    #[test]
    fn list_name_parsing() {
        let expected_result = "List name test";
        let matches = create_test_app().get_matches_from(&["test", "-n", expected_result]);
        let actual_result = get_list_name(&matches).unwrap();

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn task_title_parsing() {
        let expected_result = "Task title test";
        let matches = create_test_app().get_matches_from(&["test", "-t", expected_result]);
        let actual_result = get_task_title(&matches).unwrap();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn task_description_parsing() {
        let expected_result = "Task description test";
        let matches = create_test_app().get_matches_from(&["test", "-d", expected_result]);
        let actual_result = get_task_description(&matches).unwrap();

        assert_eq!(expected_result, actual_result)
    }
}
