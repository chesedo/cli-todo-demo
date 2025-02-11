use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub enum Command {
    /// Add a new task to the TODO list
    Add,

    /// List all tasks in the TODO list
    List,

    /// Mark a task as complete
    Complete,
}
