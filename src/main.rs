mod cli;
mod tasks;

// use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufReader};
use tasks::{Program, TaskItem, TaskList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap::App::new("doit")
        .subcommand(
            clap::SubCommand::with_name("add")
                .subcommand(
                    clap::SubCommand::with_name("list")
                        .arg(clap::Arg::with_name("name").short("n").takes_value(true)),
                )
                .subcommand(
                    clap::SubCommand::with_name("task")
                        .arg(clap::Arg::with_name("title").short("t").takes_value(true))
                        .arg(
                            clap::Arg::with_name("description")
                                .short("d")
                                .takes_value(true),
                        )
                        .arg(clap::Arg::with_name("list").short("l").takes_value(true)),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("rm")
                .subcommand(
                    clap::SubCommand::with_name("task")
                        .arg(clap::Arg::with_name("list").short("l").takes_value(true))
                        .arg(clap::Arg::with_name("index").short("i").takes_value(true)),
                )
                .subcommand(
                    clap::SubCommand::with_name("list")
                        .arg(clap::Arg::with_name("name").short("n").takes_value(true)),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("show")
                .subcommand(
                    clap::SubCommand::with_name("list")
                        .arg(clap::Arg::with_name("name").short("n").takes_value(true)),
                )
                .subcommand(clap::SubCommand::with_name("all")),
        );

    // let filepath = "tasks.json";

    // let mut program = Program::from_json_file(&File::open(&filepath)?)?;
    let mut program = Program::new(&HashMap::default());
    program.add_list(&"b".to_string(), &TaskList::new(&[]));

    match app.get_matches().subcommand() {
        ("add", Some(add_matches)) => match add_matches.subcommand() {
            ("task", Some(task_matches)) => {
                let (list_name, task) = cli::parse_add_task_command(task_matches)?;

                program
                    .add_task(&list_name, &task)
                    .expect(&format!("No list with name \"{}\" found", list_name));
            }
            ("list", Some(list_matches)) => {
                let (list_name, list) = cli::parse_add_list_command(list_matches)?;

                program.add_list(&list_name, &list);
            }
            ("", None) => {}
            _ => unreachable!(),
        },
        ("rm", Some(rm_matches)) => match rm_matches.subcommand() {
            ("task", Some(task_matches)) => {}
            ("list", Some(list_matches)) => {}
            ("", None) => {}
            _ => unreachable!(),
        },
       ("show", Some(show_matches)) => match show_matches.subcommand() {
           ("list", Some(list_matches)) => {},
           ("all", Some(all_matches)) => {},
            ("", None) => {}
            _ => unreachable!(),
        }
        ("", None) => {}
        _ => unreachable!(),
    }

    println!("{:#?}", program);

    Ok(())
}
