use clap::Parser;
use cli::Command;
use task::TaskManager;

mod cli;
mod storage;
mod task;

fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    let mut task_manager = TaskManager::new()?;

    match command {
        Command::Add { title, priority } => {
            task_manager.add(title, priority)?;

            println!("Successfully added task!");
            println!();

            task_manager.list()?;
        }
        Command::List => task_manager.list()?,
        Command::Complete { index } => {
            task_manager.complete(index)?;

            println!("Successfully completed task!");
            println!();

            task_manager.list()?;
        }
    }

    Ok(())
}
