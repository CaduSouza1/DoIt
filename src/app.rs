use clap::{App, Arg, SubCommand};

pub fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("doit")
        .arg(
            Arg::with_name("save-file")
                .short("s")
                .takes_value(true)
                .help("Sets the file where the data will be stored"),
        )
        .arg(
            Arg::with_name("read-file")
                .short("r")
                .takes_value(true)
                .help("Sets the file where the data will be read"),
        )
        .subcommands(vec![
            SubCommand::with_name("add")
                .about("Adds tasks to existing lists or creates new empty lists")
                .alias("a")
                .subcommands(vec![
                    SubCommand::with_name("task")
                        .about("Adds a task to an existing list")
                        .args(&[
                            Arg::with_name("title")
                                .short("t")
                                .takes_value(true)
                                .help("Sets the name of the task to be created")
                                .required(true),
                            Arg::with_name("description")
                                .short("d")
                                .takes_value(true)
                                .help("Sets the description of the task to be created")
                                .default_value(""),
                            Arg::with_name("list")
                                .short("l")
                                .takes_value(true)
                                .help("Sets the list where the task will be stored")
                                .required(true),
                            Arg::with_name("index")
                                .short("i")
                                .takes_value(true)
                                .help("Sets the index where the task will be stored in the list")
                                .required(false)
                                .default_value("0"),
                        ]),
                    SubCommand::with_name("list")
                        .about("Creates a new empty task list")
                        .args(&[
                            Arg::with_name("list")
                                .short("l")
                                .takes_value(true)
                                .help("Sets the name of the list to be created")
                                .required(true),
                            Arg::with_name("index")
                                .short("i")
                                .takes_value(true)
                                .help("Sets the index where the list will be stored")
                                .default_value("0"),
                        ]),
                ]),
            SubCommand::with_name("remove")
                .about("Removes tasks from existing lists or removes an entire list")
                .alias("r")
                .subcommands(vec![
                    SubCommand::with_name("task")
                        .about("Removes a task from an existing list")
                        .alias("t")
                        .args(&[
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
                    SubCommand::with_name("list")
                        .about("Removes an entire list")
                        .alias("l")
                        .args(&[
                            Arg::with_name("list")
                                .short("l")
                                .takes_value(true)
                                .help("Sets the list name to be deleted"),
                            Arg::with_name("index")
                                .short("i")
                                .takes_value(true)
                                .help("Sets the index where the list is stored for deletion"),
                        ]),
                ]),
            SubCommand::with_name("show")
                .about("Shows task information of a list")
                .alias("s")
                .subcommands(vec![
                    SubCommand::with_name("list")
                        .about("Shows the information of all tasks of a specific list")
                        .args(&[
                            Arg::with_name("list")
                                .short("l")
                                .takes_value(true)
                                .help("Sets the list name to be shown"),
                            Arg::with_name("index")
                                .short("i")
                                .takes_value(true)
                                .help("Sets the index where the list is stored to be shown"),
                        ]),
                    SubCommand::with_name("all")
                        .about("Shows the information of all tasks of all lists"),
                ]),
            SubCommand::with_name("complete")
                .alias("c")
                .about("Marks a task or an entire list as completed")
                .subcommands(vec![
                    SubCommand::with_name("list")
                        .about("Marks all tasks in a list as completed")
                        .args(&[
                            Arg::with_name("list")
                                .short("l")
                                .takes_value(true)
                                .help("Sets the list name to mark all tasks as completed"),
                            Arg::with_name("index")
                                .short("i")
                                .takes_value(true)
                                .help("Sets the index where the list is stored"),
                        ]),
                    SubCommand::with_name("task")
                        .about("Marks a task as completed")
                        .alias("t")
                        .args(&[
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
                                .help("Sets the list where the task will be marked as completed")
                                .required(true),
                            Arg::with_name("index")
                                .short("i")
                                .takes_value(true)
                                .help("Sets the index where the task is stored"),
                        ]),
                ]),
        ])
}
