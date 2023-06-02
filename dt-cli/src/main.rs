use std::io;
use std::path::PathBuf;

use clap::Parser;

use dev_tracker_core::data::DataStore;

mod cli;
mod ops;

const APP_NAME: &'static str = "dev-tracker";

fn main() -> anyhow::Result<()> {
    if !default_file_directory().exists() {
        create_default_file_dir()?;
    }
    let ds = DataStore::new(Some(&default_data_file_path()))?;

    let args = cli::Arguments::parse();

    match args.command {
        cli::Command::Add(command) => match command {
            cli::AddCommand::Project(args) => ops::add_project(args, &ds)?,
        },
        cli::Command::Delete(command) => match command {
            cli::DeleteCommand::Project(args) => ops::delete_project(args, &ds)?,
        },
        cli::Command::Describe(command) => match command {
            cli::DescribeCommand::Project(args) => ops::describe_project(args, &ds)?,
        },
        cli::Command::List(command) => match command {
            cli::ListCommand::Projects => ops::list_projects(&ds)?,
        },
        cli::Command::Rename(command) => match command {
            cli::RenameCommand::Project(args) => ops::rename_project(args, &ds)?,
        },
        cli::Command::Update(command) => match command {
            cli::UpdateCommand::Project(args) => ops::update_project(args, &ds)?,
        },
    }

    Ok(())
}

fn create_default_file_dir() -> io::Result<()> {
    std::fs::create_dir_all(default_file_directory())?;
    Ok(())
}

fn default_file_directory() -> PathBuf {
    let mut path: PathBuf = dirs::config_dir().expect("should be able to get config directory.");
    path.push(APP_NAME);
    path
}

fn default_data_file_path() -> PathBuf {
    let mut path = default_file_directory();
    path.push("dev-tracker");
    path.set_extension("sqlite");
    path
}

// fn default_config_file_path() -> PathBuf {
//     let mut path = default_file_directory();
//     path.push("config");
//     path.set_extension("toml");
//     path
// }
