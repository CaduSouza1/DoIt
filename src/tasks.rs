// TODO: Create an id system to retrieve objects easily.
// TODO: Remove Option<&mut TaskItem> as the return type of get_task()
#[derive(Debug)]
pub struct TaskItem {
    title: String,
    description: String,
    is_completed: bool,
    subtasks: Vec<TaskItem>,
}

impl TaskItem {
    pub fn new(title: &str, description: &str) -> TaskItem {
        return TaskItem {
            title: title.to_string(),
            description: description.to_string(),
            is_completed: false,
            subtasks: Vec::new(),
        };
    }

    pub fn add_child(&mut self, child: TaskItem) {
        self.subtasks.push(child);
    }

    pub fn get_task(&mut self, index: usize) -> Option<&mut TaskItem> {
        return if self.subtasks.is_empty() {
            None
        } else {
            Some(&mut self.subtasks[index])
        };
    }

    pub fn format_tasks(&self, depth: usize, max_depth: usize) -> String {
        return format!(
            "\n{}[{}] {} {} {}",
            std::iter::repeat("\t").take(depth).collect::<String>(),
            if self.is_completed { "O" } else { "X" },
            self.title,
            self.description,
            if depth == max_depth || self.subtasks.is_empty() {
                "".to_string()
            } else {
                self.subtasks
                    .iter()
                    .map(|item| item.format_tasks(depth + 1, max_depth))
                    .collect()
            }
        );
    }
}

#[derive(Debug)]
pub struct TaskList {
    name: String,
    tasks:Vec<TaskItem>,
}

impl TaskList {
    pub fn new(name: &str) -> TaskList {
        return TaskList {
            name: name.to_string(),
            tasks: Vec::new(),
        };
    }

    pub fn add_task(&mut self, task: TaskItem) {
        self.tasks.push(task);
    }

    pub fn get_task(&self, index: usize) -> &TaskItem {
        return &self.tasks[index];
    }

    pub fn change_task(&mut self, index: usize, new_task: TaskItem, keep_subtasks: bool) {
        self.tasks[index].title = new_task.title;
        self.tasks[index].description = new_task.description;
        self.tasks[index].is_completed = new_task.is_completed;

        if !keep_subtasks {
            self.tasks[index].subtasks = vec![]
        }
    }

    pub fn format_tasks(&self, depth: usize, max_depth: usize) -> String {
        return format!(
            "{}: {}",
            self.name,
            self.tasks
                .iter()
                .map(|task| task.format_tasks(depth + 1, max_depth))
                .collect::<String>()
        );
    }
}
