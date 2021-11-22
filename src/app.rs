use clap::{App, Arg, SubCommand};

pub fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("doit").subcommands(vec![
        SubCommand::with_name("add").alias("a").subcommands(vec![
            SubCommand::with_name("task").args(&[
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
            SubCommand::with_name("list").args(&[
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
