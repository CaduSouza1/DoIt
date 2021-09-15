extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

impl TaskItem {
    pub fn new(title: &str, description: &str, is_completed: bool) -> Self {
        return Self {
            title: title.to_string(),
            description: description.to_string(),
            is_completed: is_completed,
        };
    }

    pub fn format(&self, depth: usize) -> String {
        return format!(
            "{}[{}] {} \n{}{}\n\n",
            std::iter::repeat("\t").take(depth).collect::<String>(),
            if self.is_completed { "X" } else { " " },
            self.title,
            std::iter::repeat("\t").take(depth + 1).collect::<String>(),
            self.description
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<TaskItem>,
}

impl TaskList {
    pub fn new(tasks: &[TaskItem]) -> Self {
        Self {
            tasks: tasks.to_vec(),
        }
    }

    pub fn format_tasks(&self, depth: usize) -> String {
        return format!(
            "{}\n",
            self.tasks
                .iter()
                .map(|task| task.format(depth))
                .collect::<String>()
        );
    }

    pub fn add_task(&mut self, task: &TaskItem) {
        self.tasks.push(task.clone())
    }

    pub fn remove_task(&mut self, index: usize) -> TaskItem {
        self.tasks.remove(index)
    }
}

// I don't know what to name this struct, so I'm just going to leave this as it is for now
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub tasks_lists: HashMap<String, TaskList>,
}

impl Program {
    pub fn new(tasks_lists: &HashMap<String, TaskList>) -> Self {
        Self {
            tasks_lists: tasks_lists.clone(),
        }
    }

    pub fn from_json_file(file: &std::fs::File) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(std::io::BufReader::new(file))
    }

    pub fn add_list(&mut self, list_name: &String, list: &TaskList) -> Option<TaskList> {
        self.tasks_lists.insert(list_name.clone(), list.clone())
    }

    pub fn add_task(&mut self, list_name: &String, task: &TaskItem) -> Option<()> {
        Some(self.tasks_lists.get_mut(list_name)?.add_task(task))
    }

    pub fn remove_list(&mut self, list_name: &String) -> Option<TaskList> {
        self.tasks_lists.remove(list_name)
    }

    pub fn remove_task(&mut self, list_name: &String, task_index: usize) -> Option<TaskItem> {
        Some(self.tasks_lists.get_mut(list_name)?.remove_task(task_index))
    }

    pub fn format_task_list(&self, list_name: &String, depth: usize) -> Option<String> {
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
}
