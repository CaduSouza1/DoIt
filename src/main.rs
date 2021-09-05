mod cli;
mod tasks;

use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufReader};
use tasks::{Program, TaskItem, TaskList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap::App::new("DoIt")
    .after_help("The filepath and list names are reused after the first time using them. So it is not necessary to always specify these arguments")
    .arg(
        clap::Arg::with_name("filepath")
        .short("f")
        .long("file")
        .help("Sets the file to make the necessary operations")
        .value_name("Filepath"),
    )
    .arg(
        clap::Arg::with_name("new-list")
        .short("n")
        .long("new-list")
        .help("Creates a new task list with a specified by the -l command")
    )
    .arg(
        clap::Arg::with_name("add-task")
        .short("a")
        .long("add-task")
        .help("Adds a new task and appends it to a task list")
        .takes_value(true)
        .multiple(true)
        .value_names(&["Title", "Description", "Is completed"])
    )
    .arg(
        clap::Arg::with_name("delete-list")
        .short("d")
        .long("delete-list")
        .help("Deletes a list specified by the -l command")
    )
    .arg(
        clap::Arg::with_name("remove-task")
        .short("r")
        .long("remove-task")
        .help("Removes a task at a specific index on a task list")
        .takes_value(true)
        .value_name("Index")
    )
    .arg(
        clap::Arg::with_name("list")
        .short("l")
        .long("list")
        .help("Sets the task list to make the necessary operations")
        .takes_value(true)
        .value_name("List name")
    )
    .arg(
        clap::Arg::with_name("show")
        .short("s")
        .long("show")
        .help("Shows all created tasks of a particular list")
    )
    .arg(
        clap::Arg::with_name("show-all")
        .long("show-all")
        .help("Shows all created tasks")
    );

    let defaults_filepath = "defaults.json";

    let mut defaults: HashMap<String, String> =
        serde_json::from_reader(BufReader::new(&File::open(&defaults_filepath)?))?;

    let matches = app.get_matches();

    let filepath = match matches.value_of("filepath") {
        Some(path) => path.to_string(),
        None => {
            let path = defaults.get("last_filepath").unwrap();
            println!("No filepath given. Using last filepath \"{}\"", path);

            path.clone()
        }
    };

    let mut program = Program::load_from_json_file(&File::open(&filepath)?)?;

    let list_name = match matches.value_of("list") {
        Some(name) => name.to_string(),
        None => {
            let name = defaults.get("last_list_name").unwrap();
            println!("No list name given. Using default list name \"{}\"", name);
            name.clone()
        }
    };

    matches.values_of("add-task").and_then(|mut args| {
        println!("Added task to list {}", list_name);
        program.add_task(
            &list_name,
            TaskItem::new(args.nth(0).unwrap(), args.nth(0).unwrap(), false),
        )
    });

    matches.value_of("remove-task").and_then(|index| {
        println!("Removed task {} from list {}", index, list_name);
        program.remove_task(&list_name, index.parse().unwrap())
    });
    
    if matches.is_present("new-list") {
        println!("Created list with name {}", list_name);
        program.add_list(list_name.clone(), TaskList::default());
    };
    
    if matches.is_present("delete-list") {
        println!("Deleted list with name {}", list_name);
        program.remove_list(&list_name);
    };

    if matches.is_present("show") {
        println!("{}", program.format_task_list(&list_name, 1).unwrap());
    }

    if matches.is_present("show-all") {
        println!("{}", program.format_all_tasks(1).unwrap());
    }

    defaults
        .insert("last_filepath".to_string(), filepath.clone())
        .and_then(|old_filepath| {
            if old_filepath != filepath {
                println!(
                    "Updated default filepath from {} to {}",
                    old_filepath, filepath
                )
            }

            Some(())
        });

    defaults
        .insert("last_list_name".to_string(), list_name.clone())
        .and_then(|old_list_name| {
            if old_list_name != list_name {
                println!(
                    "Updated default list name from {} to {}",
                    old_list_name, list_name
                )
            }

            Some(())
        });

    let mut program_file_write = File::create(&filepath)?;
    let mut defaults_file_write = File::create(&defaults_filepath)?;

    program_file_write.write_all(serde_json::to_string_pretty(&program)?.as_bytes())?;
    defaults_file_write.write_all(serde_json::to_string_pretty(&defaults)?.as_bytes())?;

    Ok(())
}
