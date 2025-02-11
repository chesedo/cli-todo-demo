use std::{
    fs::{read_to_string, write},
    path::Path,
};

use crate::task::Task;

pub struct FileStorage;

impl FileStorage {
    const FILENAME: &str = "tasks.json";

    pub fn new() -> Self {
        if !Path::new(Self::FILENAME).exists() {
            std::fs::write(Self::FILENAME, "[]").unwrap();
        }

        Self
    }

    pub fn load(&self) -> Vec<Task> {
        let json = read_to_string(Self::FILENAME).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    pub fn save(&self, tasks: &[Task]) {
        let json = serde_json::to_string(tasks).unwrap();
        write(Self::FILENAME, json).unwrap();
    }
}
