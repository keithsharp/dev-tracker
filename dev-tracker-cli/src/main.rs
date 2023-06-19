use std::io;
use std::path::PathBuf;

use clap::Parser;

use dev_tracker_core::data::DataStore;

mod cli;
mod ops;

const APP_NAME: &str = "dev-tracker";

fn main() -> anyhow::Result<()> {
    let args = cli::Arguments::parse();

    let ds = match args.data_file {
        Some(path) => DataStore::new(Some(&path))?,
        None => {
            if !default_file_directory().exists() {
                create_default_file_dir()?;
            }
            DataStore::new(Some(&default_data_file_path()))?
        }
    };

    match args.command {
        cli::Command::Add(command) => match command {
            cli::AddCommand::Project(args) => ops::add_project(args, &ds)?,
            cli::AddCommand::ActivityType(args) => ops::add_activitytype(args, &ds)?,
            cli::AddCommand::Repo(args) => ops::add_repo(args, &ds)?,
        },
        cli::Command::Delete(command) => match command {
            cli::DeleteCommand::Project(args) => ops::delete_project(args, &ds)?,
            cli::DeleteCommand::Activity(args) => ops::delete_activity(args, &ds)?,
            cli::DeleteCommand::ActivityType(args) => ops::delete_activitytype(args, &ds)?,
            cli::DeleteCommand::Repo(args) => ops::delete_repo(args, &ds)?,
            cli::DeleteCommand::Count(args) => ops::delete_count(args, &ds)?,
        },
        cli::Command::Count(args) => ops::count(args, &ds)?,
        cli::Command::Cancel(command) => match command {
            cli::CancelCommand::Activity(args) => ops::cancel_actvity(args, &ds)?,
        },
        cli::Command::Describe(command) => match command {
            cli::DescribeCommand::Project(args) => ops::describe_project(args, &ds)?,
            cli::DescribeCommand::Activity(args) => ops::describe_activity(args, &ds)?,
            cli::DescribeCommand::Count(args) => ops::describe_count(args, &ds)?,
        },
        cli::Command::List(command) => match command {
            cli::ListCommand::Projects(args) => ops::list_projects(args, &ds)?,
            cli::ListCommand::Activities(args) => ops::list_activities(args, &ds)?,
            cli::ListCommand::ActivityTypes(args) => ops::list_activitytypes(args, &ds)?,
            cli::ListCommand::Repos(args) => ops::list_repos(args, &ds)?,
            cli::ListCommand::Counts(args) => ops::list_counts(args, &ds)?,
        },
        cli::Command::Rename(command) => match command {
            cli::RenameCommand::Project(args) => ops::rename_project(args, &ds)?,
            cli::RenameCommand::ActivityType(args) => ops::rename_activitytype(args, &ds)?,
        },
        cli::Command::Start(command) => match command {
            cli::StartCommand::Activity(args) => ops::start_activity(args, &ds)?,
        },
        cli::Command::Stop(command) => match command {
            cli::StopCommand::Activity(args) => ops::stop_activity(args, &ds)?,
        },
        cli::Command::Update(command) => match command {
            cli::UpdateCommand::ActivityType(args) => ops::update_activitytype(args, &ds)?,
            cli::UpdateCommand::Activity(args) => match args.command {
                cli::UpdateActivityCommand::Description(args) => {
                    ops::update_activity_description(args, &ds)?
                }
                cli::UpdateActivityCommand::End(args) => ops::update_activity_end(args, &ds)?,
                cli::UpdateActivityCommand::Project(args) => {
                    ops::update_activity_project(args, &ds)?
                }
                cli::UpdateActivityCommand::ActivityType(args) => {
                    ops::update_activity_atype(args, &ds)?
                }
            },
            cli::UpdateCommand::Repo(args) => ops::update_repo(args, &ds)?,
        },
        cli::Command::Generate(command) => match command {
            cli::GenerateCommand::Report(args) => ops::generate_report(args, &ds)?,
            cli::GenerateCommand::Json(args) => ops::generate_json(args, &ds)?,
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
