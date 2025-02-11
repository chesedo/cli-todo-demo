use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::storage::FileStorage;

#[derive(Deserialize, Serialize)]
pub struct Task {
    title: String,
    completed: bool,
}

pub struct TaskManager {
    storage: FileStorage,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            storage: FileStorage::new(),
        }
    }

    pub fn add(&mut self, title: String) {
        let mut tasks = self.storage.load();

        tasks.push(Task {
            title,
            completed: false,
        });

        self.storage.save(&tasks);
    }

    pub fn list(&self) {
        let tasks = self.storage.load();

        if tasks.is_empty() {
            println!("{}", "No tasks found".yellow());
            return;
        }

        println!("{}", "Tasks list:".green());

        for (index, task) in tasks.into_iter().enumerate() {
            let status = if task.completed { "✓" } else { "◌" };

            println!("[{}] - {}: {}", index + 1, status, task.title);
        }
    }

    pub fn complete(&mut self, index: usize) -> Result<(), &str> {
        let mut tasks = self.storage.load();
        let task = tasks.get_mut(index - 1).ok_or("Task not found")?;

        task.completed = true;

        self.storage.save(&tasks);

        Ok(())
    }
}
