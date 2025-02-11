use colored::Colorize;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::storage::{self, FileStorage};

#[derive(Deserialize, Serialize)]
pub struct Task {
    title: String,
    completed: bool,

    #[serde(default = "default_priority")]
    priority: u8,
}

fn default_priority() -> u8 {
    1
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Storage failed")]
    Storage(#[from] storage::Error),

    #[error("Task not found")]
    NotFound,
}

pub struct TaskManager {
    storage: FileStorage,
}

impl TaskManager {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            storage: FileStorage::new()?,
        })
    }

    pub fn add(&mut self, title: String, priority: u8) -> Result<(), Error> {
        let mut tasks = self.storage.load()?;

        tasks.push(Task {
            title,
            completed: false,
            priority,
        });

        self.storage.save(&tasks)?;

        Ok(())
    }

    pub fn list(&self) -> Result<(), Error> {
        let tasks = self.storage.load()?;

        if tasks.is_empty() {
            println!("{}", "No tasks found".yellow());
            return Ok(());
        }

        println!("{}", "Tasks list:".green());

        for (index, task) in tasks.into_iter().enumerate() {
            let status = if task.completed { "✓" } else { "◌" };

            println!(
                "[{}] - {}: ({}) {}",
                index + 1,
                status,
                task.priority,
                task.title
            );
        }

        Ok(())
    }

    pub fn complete(&mut self, index: usize) -> Result<(), Error> {
        let mut tasks = self.storage.load()?;
        let task = tasks.get_mut(index - 1).ok_or(Error::NotFound)?;

        task.completed = true;

        self.storage.save(&tasks)?;

        Ok(())
    }
}
