#![allow(dead_code)]

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

impl TaskItem {
    pub fn new(title: &str, description: &str, is_completed: bool) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            is_completed,
        }
    }

    pub fn format(&self, depth: usize) -> String {
        format!(
            "{}[{}] {} \n{}{}\n\n",
            "\t".repeat(depth),
            if self.is_completed { "X" } else { " " },
            self.title,
            "\t".repeat(depth + 1),
            self.description
        )
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<TaskItem>,
}

impl TaskList {
    pub fn new(tasks: &[TaskItem]) -> Self {
        Self {
            tasks: tasks.to_vec(),
        }
    }

    pub fn format_tasks(&self, depth: usize) -> String {
        format!(
            "{}\n",
            self.tasks
                .iter()
                .map(|task| task.format(depth))
                .collect::<String>()
        )
    }

    pub fn add_task(&mut self, task: &TaskItem) {
        self.tasks.push(task.clone())
    }

    pub fn remove_task(&mut self, index: usize) -> TaskItem {
        self.tasks.remove(index)
    }

    pub fn find_task_with_title(&self, title: &str) -> Option<(usize, &TaskItem)> {
        self.tasks
            .iter()
            .enumerate()
            .find(|task| task.1.title == title)
    }
}

// I don't know what to name this struct, so I'm just going to leave this as it is for now
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskLists {
    pub tasks_lists: HashMap<String, TaskList>,
}

impl TaskLists {
    pub fn new(tasks_lists: &HashMap<String, TaskList>) -> Self {
        Self {
            tasks_lists: tasks_lists.clone(),
        }
    }

    pub fn from_json_file(file: &std::fs::File) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(std::io::BufReader::new(file))
    }

    pub fn add_list(&mut self, list_name: &str, list: &TaskList) -> Option<TaskList> {
        self.tasks_lists.insert(list_name.to_string(), list.clone())
    }

    pub fn add_task(&mut self, list_name: &str, task: &TaskItem) -> Option<()> {
        self.tasks_lists.get_mut(list_name)?.add_task(task);
        Some(())
    }

    pub fn remove_list(&mut self, list_name: &str) -> Option<TaskList> {
        self.tasks_lists.remove(list_name)
    }

    pub fn remove_task(&mut self, list_name: &str, task_index: usize) -> Option<TaskItem> {
        Some(self.tasks_lists.get_mut(list_name)?.remove_task(task_index))
    }

    pub fn format_task_list(&self, list_name: &str, depth: usize) -> Option<String> {
        Some(format!(
            "{}: \n{}\n",
            list_name,
            self.tasks_lists.get(list_name)?.format_tasks(depth)
        ))
    }

    pub fn format_all_tasks(&self, depth: usize) -> String {
        self.tasks_lists
            .iter()
            .map(|(list_name, list)| format!("{}: \n{}\n", list_name, list.format_tasks(depth)))
            .collect()
    }

    pub fn complete_task(&mut self, list_name: &str, task_index: usize) -> Option<()> {
        self.tasks_lists.get_mut(list_name)?.tasks[task_index].is_completed = true;
        Some(())
    }

    pub fn complete_list(&mut self, list_name: &str) -> Option<()> {
        self.tasks_lists
            .get_mut(list_name)?
            .tasks
            .iter_mut()
            .for_each(|task| task.is_completed = true);
        Some(())
    }
}
