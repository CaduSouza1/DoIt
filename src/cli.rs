use crate::tasks;

use tasks::{TaskItem, TaskList};

pub fn parse_add_list_command(
    matches: &clap::ArgMatches,
) -> Result<(String, TaskList), Box<dyn std::error::Error>> {
    Ok((
        matches
            .value_of("name")
            .ok_or("No list name given")?
            .to_string(),
        TaskList::new(&[]),
    ))
}

pub fn parse_add_task_command(
    matches: &clap::ArgMatches,
) -> Result<(String, TaskItem), Box<dyn std::error::Error>> {
    Ok((
        matches
            .value_of("list")
            .ok_or("No list name given")?
            .to_string(),
        TaskItem::new(
            matches.value_of("title").ok_or("No title given")?,
            matches.value_of("description").ok_or("")?,
            false,
        ),
    ))
}

pub fn remove_task_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn remove_list_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
