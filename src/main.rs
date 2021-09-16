#[macro_use]

mod cli;
mod tasks;

extern crate clap;

// use std::io::Write;
use std::collections::HashMap;
use tasks::{Program, TaskItem, TaskList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = clap::load_yaml!("cli.yml");
    let app = clap::App::from_yaml(yaml);

    let mut program = Program::new(&HashMap::default());
    program.add_list(&"b".to_string(), &TaskList::new(&[]));

    match app.get_matches().subcommand() {
        ("add", Some(add_matches)) => match add_matches.subcommand() {
            ("task", Some(task_matches)) => {
                program
                    .add_task(
                        &cli::get_list_name(task_matches)?,
                        &TaskItem::new(
                            &cli::get_task_title(task_matches)?,
                            &cli::get_task_description(task_matches)?,
                            false,
                        ),
                    )
                    .expect("No list with given name found");
            }
            ("list", Some(list_matches)) => {
                program.add_list(&cli::get_list_name(list_matches)?, &TaskList::new(&[]));
            }
            ("", None) => {}
            _ => unreachable!(),
        },
        ("rm", Some(rm_matches)) => match rm_matches.subcommand() {
            ("task", Some(task_matches)) => {
                program.remove_task(
                    &cli::get_list_name(task_matches)?,
                    cli::get_task_index(task_matches)?,
                );
            }
            ("list", Some(list_matches)) => {
                program.remove_list(&cli::get_list_name(list_matches)?);
            }
            ("", None) => {}
            _ => unreachable!(),
        },
        ("show", Some(show_matches)) => match show_matches.subcommand() {
            ("list", Some(list_matches)) => {}
            ("all", Some(all_matches)) => {}
            ("", None) => {}
            _ => unreachable!(),
        },
        ("", None) => {}
        _ => unreachable!(),
    }

    println!("{:#?}", program);

    Ok(())
}
