use std::{
    fs::{read_to_string, write},
    path::Path,
};

use thiserror::Error;

use crate::task::Task;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to Serialize/Deserialize JSON")]
    Serde(#[from] serde_json::Error),

    #[error("Failed to read/write file")]
    Io(#[from] std::io::Error),
}

pub struct FileStorage;

impl FileStorage {
    const FILENAME: &str = "tasks.json";

    pub fn new() -> Result<Self, Error> {
        if !Path::new(Self::FILENAME).exists() {
            std::fs::write(Self::FILENAME, "[]")?;
        }

        Ok(Self)
    }

    pub fn load(&self) -> Result<Vec<Task>, Error> {
        let json = read_to_string(Self::FILENAME)?;
        let tasks = serde_json::from_str(&json)?;

        Ok(tasks)
    }

    pub fn save(&self, tasks: &[Task]) -> Result<(), Error> {
        let json = serde_json::to_string(tasks)?;
        write(Self::FILENAME, json)?;

        Ok(())
    }
}
