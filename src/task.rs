use colored::Colorize;

pub struct Task {
    title: String,
    completed: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: vec![Task {
                title: "Finish the task manager".to_string(),
                completed: false,
            }],
        }
    }

    pub fn add(&mut self, title: String) {
        self.tasks.push(Task {
            title,
            completed: false,
        });
    }

    pub fn list(&self) {
        println!("{}", "Tasks list:".green());

        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { "◌" };

            println!("[{}] - {}: {}", index + 1, status, task.title);
        }
    }

    pub fn complete(&mut self, index: usize) {
        let task = self.tasks.get_mut(index - 1).unwrap();

        task.completed = true;
    }
}
