use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub enum Command {
    /// Add a new task to the TODO list
    Add {
        /// Title of task to add
        title: String,
    },

    /// List all tasks in the TODO list
    List,

    /// Mark a task as complete
    Complete {
        /// Number of the task to complete
        index: usize,
    },
}
