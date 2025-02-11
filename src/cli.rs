use clap::{value_parser, Parser};

#[derive(Parser)]
#[command(version, about)]
pub enum Command {
    /// Add a new task to the TODO list
    Add {
        /// Title of task to add
        title: String,

        #[arg(short, long, value_parser = value_parser!(u8).range(1..=5))]
        /// The priority of the task
        priority: u8,
    },

    /// List all tasks in the TODO list
    List,

    /// Mark a task as complete
    Complete {
        /// Number of the task to complete
        index: usize,
    },
}
