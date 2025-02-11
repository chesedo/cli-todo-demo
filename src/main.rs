use clap::Parser;
use cli::Command;

mod cli;

fn main() {
    let command = Command::parse();

    match command {
        Command::Add => todo!(),
        Command::List => todo!(),
        Command::Complete => todo!(),
    }
}
