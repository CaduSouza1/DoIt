mod cli;
mod tasks;

use clap::{App, Arg, SubCommand};
use std::io;
use std::io::Write;
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
        ("add", Some(add_matches)) => cli::parse_add_command(add_matches, &mut program),
        ("remove", Some(rm_matches)) => cli::parse_remove_command(rm_matches, &mut program),
        ("show", Some(show_matches)) => cli::parse_show_command(show_matches, &program),
        ("complete", Some(complete_matches)) => {
            cli::parse_complete_command(complete_matches, &mut program)
        }
        ("", None) => {}
        _ => unreachable!(),
    }

    std::fs::File::create("tasks.json")?
        .write_all(serde_json::to_string_pretty(&program)?.as_bytes())?;

    Ok(())
}
