pub struct TaskItem {
    title: String,
    description: String,
    is_completed: bool,   
}

impl TaskItem {
    pub fn new(title: &str, description: &str, is_completed: bool) -> TaskItem {
        return TaskItem {
            title: title.to_string(),
            description: description.to_string(),
            is_completed: is_completed
        };
    }

   pub fn format(&self, depth: usize) -> String {
        return format!(
            "{}[{}] {} {}",
            std::iter::repeat("\t").take(depth).collect::<String>(),
            if self.is_completed { "X" } else { " " },
            self.title,
            self.description,   
        )
    }
}

pub struct TaskList {
    name: String,
    tasks: Vec<TaskItem>
}

impl TaskList {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            tasks: Vec::new()
        }
    }

    pub fn format_tasks(&self, depth: usize) -> String {
        return format!(
            "{}: \n{}",
            self.name,
            self.tasks.iter().map(|task| task.format(depth)).collect::<String>()
        )
    }

    pub fn add_task(&mut self, task: TaskItem) {
        self.tasks.push(task)
    }

    pub fn remove_task(&mut self, index: usize) -> TaskItem {
        self.tasks.remove(index)
    }
}