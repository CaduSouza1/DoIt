mod tasks;

use tasks::{TaskItem, TaskList};

fn main() {
    let mut tl = TaskList::new("Test");

    tl.add_task(
        TaskItem::new("title: &str", "description: &str", true)
    );

    println!("{}", tl.format_tasks(1));

    tl.remove_task(0);
    
    println!("{}", tl.format_tasks(1));
}
