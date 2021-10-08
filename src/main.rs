#[macro_use]

mod cli;
mod tasks;

extern crate clap;

// use std::io::Write;
use std::collections::HashMap;
use clap::{App, SubCommand, Arg};
use tasks::{Program, TaskItem, TaskList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "tasks.json";

    let app = App::new("doit").subcommands(vec![
        SubCommand::with_name("add").alias("a").subcommands(vec![
            SubCommand::with_name("task").args(&[
                Arg::with_name("title")
                    .short("t")
                    .takes_value(true)
                    .help("Sets the name of the task to be created"),
                Arg::with_name("description")
                    .short("d")
                    .takes_value(true)
                    .help("Sets the description of the task to be created"),
                Arg::with_name("list")
                    .short("l")
                    .takes_value(true)
                    .help("Sets the list where the task will be stored"),
                Arg::with_name("index")
                    .short("i")
                    .takes_value(true)
                    .help("Sets the index where the task will be stored in the list"),
            ]),
            SubCommand::with_name("list").args(&[
                Arg::with_name("name")
                    .short("n")
                    .takes_value(true)
                    .help("Sets the name of the list to be created"),
                Arg::with_name("index")
                    .short("i")
                    .takes_value(true)
                    .help("Sets the index where the list will be stored in the program"),
            ]),
        ]),
        SubCommand::with_name("remove").alias("r").subcommands(vec![
            SubCommand::with_name("task").alias("t").args(&[
                Arg::with_name("title")
                    .short("t")
                    .takes_value(true)
                    .help("Sets the name of the task to be deleted"),
                Arg::with_name("description")
                    .short("d")
                    .takes_value(true)
                    .help("Sets the description of the task to be deleted"),
                Arg::with_name("list")
                    .short("l")
                    .takes_value(true)
                    .help("Sets the list where the task will be deleted"),
                Arg::with_name("index")
                    .short("i")
                    .takes_value(true)
                    .help("Sets the index where the task is stored for deletion"),
            ]),
            SubCommand::with_name("list").alias("l").args(&[
                Arg::with_name("name")
                    .short("n")
                    .takes_value(true)
                    .help("Sets the list name to be deleted"),
                Arg::with_name("index")
                    .short("i")
                    .takes_value(true)
                    .help("Sets the index where the list is stored for deletion"),
            ]),
        ]),
        SubCommand::with_name("show").alias("s").subcommands(vec![
            SubCommand::with_name("list").args(&[
                Arg::with_name("list")
                    .short("l")
                    .takes_value(true)
                    .help("Sets the list name to be shown"),
                Arg::with_name("index")
                    .short("i")
                    .takes_value(true)
                    .help("Sets the index where the list is stored to be shown"),
            ]),
            SubCommand::with_name("all"),
        ]),
        SubCommand::with_name("complete")
            .alias("c")
            .subcommands(vec![
                SubCommand::with_name("list").args(&[
                    Arg::with_name("list")
                        .short("l")
                        .takes_value(true)
                        .help("Sets the list name to mark all tasks as completed"),
                    Arg::with_name("index")
                        .short("i")
                        .takes_value(true)
                        .help("Sets the index where the list is stored"),
                ]),
                SubCommand::with_name("task").alias("t").args(&[
                    Arg::with_name("title")
                        .short("t")
                        .takes_value(true)
                        .help("Sets the name of the task to be marked as completed"),
                    Arg::with_name("description")
                        .short("d")
                        .takes_value(true)
                        .help("Sets the description of the task to be marked as completed"),
                    Arg::with_name("list")
                        .short("l")
                        .takes_value(true)
                        .help("Sets the list where the task will be marked as completed"),
                    Arg::with_name("index")
                        .short("i")
                        .takes_value(true)
                        .help("Sets the index where the task is stored"),
                ]),
            ]),
    ]);

    let mut program = Program::from_json_file(&std::fs::File::open(&filepath)?)?;

    match app.get_matches().subcommand() {
        ("add", Some(add_matches)) => match add_matches.subcommand() {
            ("task", Some(task_matches)) => {
                program
                    .add_task(
                        &cli::get_list_name(task_matches).unwrap(),
                        &TaskItem::new(
                            &cli::get_task_title(task_matches).unwrap(),
                            &cli::get_task_description(task_matches).unwrap(),
                            false,
                        ),
                    )
                    .expect("No list with given name found");
            }
            ("list", Some(list_matches)) => {
                program.add_list(&cli::get_list_name(list_matches).unwrap(), &TaskList::new(&[]));
            }
            ("", None) => {}
            _ => unreachable!(),
        },
        ("rm", Some(rm_matches)) => match rm_matches.subcommand() {
            ("task", Some(task_matches)) => {
                program.remove_task(
                    &cli::get_list_name(task_matches).unwrap(),
                    cli::get_index(task_matches).unwrap(),
                );
            }
            ("list", Some(list_matches)) => {
                program.remove_list(&cli::get_list_name(list_matches).unwrap());
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
