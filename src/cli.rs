use crate::tasks;

use tasks::{Program, TaskItem, TaskList};

pub fn get_list_name(matches: &clap::ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    Ok(matches
        .value_of("name")
        .ok_or("No list name given")?
        .to_string())
}

pub fn get_task_title(matches: &clap::ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    Ok(matches
        .value_of("title")
        .ok_or("No title given")?
        .to_string())
}

pub fn get_task_description(
    matches: &clap::ArgMatches,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(matches
        .value_of("description")
        .ok_or("No description given")?
        .to_string())
}

pub fn get_task_index(matches: &clap::ArgMatches) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(matches.value_of("index").ok_or("No index given")?.parse()?)
}
