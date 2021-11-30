use std::path::PathBuf;
use anyhow::anyhow;
use structopt::StructOpt;
use tasks::Task;

use crate::cli::{Action::*, CommandLineArgs};

mod cli;
mod tasks;

fn main() -> anyhow::Result<()>{
    // Get command line arguments
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    // Unpack the journal file
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    // Perform the action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}

fn find_default_journal_file () -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
