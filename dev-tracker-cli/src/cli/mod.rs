use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clap::{Args, Parser, Subcommand};

mod argparser;
use argparser::*;

mod errors;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long)]
    pub config_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Add(AddCommand),
    #[clap(subcommand)]
    Cancel(CancelCommand),
    #[clap(subcommand)]
    Delete(DeleteCommand),
    #[clap(subcommand)]
    Describe(DescribeCommand),
    #[clap(subcommand)]
    List(ListCommand),
    #[clap(subcommand)]
    Rename(RenameCommand),
    // #[clap(subcommand)]
    // Set,
    #[clap(subcommand)]
    Start(StartCommand),
    #[clap(subcommand)]
    Stop(StopCommand),
    #[clap(subcommand)]
    Update(UpdateCommand),
}

#[derive(Subcommand)]
pub enum AddCommand {
    Project(AddProjectArgs),
    #[command(alias = "at")]
    ActivityType(AddActivityTypeArgs),
    Repo(AddRepoArgs),
}

#[derive(Args)]
pub struct AddProjectArgs {
    pub name: String,
    pub path: Option<PathBuf>,
}

#[derive(Args)]
pub struct AddActivityTypeArgs {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Args)]
pub struct AddRepoArgs {
    pub project: String,
    pub path: PathBuf,
}

#[derive(Subcommand)]
pub enum CancelCommand {
    Activity(CancelActivityTypeArgs),
}

#[derive(Args)]
pub struct CancelActivityTypeArgs {
    pub project: String,
}

#[derive(Subcommand)]
pub enum DeleteCommand {
    Project(DeleteProjectArgs),
    Activity(DeleteActivityArgs),
    #[command(alias = "at")]
    ActivityType(DeleteActivityTypeArgs),
    Repo(DeleteRepoArgs),
}

#[derive(Args)]
pub struct DeleteProjectArgs {
    pub name: String,
}

#[derive(Args)]
pub struct DeleteActivityArgs {
    pub id: u64,
}

#[derive(Args)]
pub struct DeleteActivityTypeArgs {
    pub name: String,
}

#[derive(Args)]
pub struct DeleteRepoArgs {
    pub path: PathBuf,
}

#[derive(Subcommand)]
pub enum DescribeCommand {
    Project(DescribeProjectArgs),
    Activity(DescribeActivityArgs),
}

#[derive(Args)]
pub struct DescribeProjectArgs {
    pub name: String,
}

#[derive(Args)]
pub struct DescribeActivityArgs {
    pub id: u64,
}

#[derive(Subcommand)]
pub enum ListCommand {
    Projects(ListProjectArgs),
    Activities(ListActivityArgs),
    #[command(alias = "ats")]
    ActivityTypes(ListActivityTypeArgs),
    Repos(ListRepoArgs),
}

#[derive(Args)]
pub struct ListProjectArgs {
    #[clap(short, action)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct ListActivityArgs {
    #[clap(short, action)]
    pub verbose: bool,
    pub project: String,
}

#[derive(Args)]
pub struct ListActivityTypeArgs {
    #[clap(short, action)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct ListRepoArgs {
    #[clap(short, action)]
    pub verbose: bool,
    pub project: String,
}

#[derive(Subcommand)]
pub enum RenameCommand {
    Project(RenameProjectArgs),
    #[command(alias = "at")]
    ActivityType(RenameActivityTypeArgs),
}

#[derive(Args)]
pub struct RenameProjectArgs {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Args)]
pub struct RenameActivityTypeArgs {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Subcommand)]
pub enum StartCommand {
    Activity(StartActivityArgs),
}

#[derive(Args)]
pub struct StartActivityArgs {
    pub project: String,
    pub activity_type: String,
    pub description: Option<String>,
}

#[derive(Subcommand)]
pub enum StopCommand {
    Activity(StopActivityArgs),
}

#[derive(Args)]
pub struct StopActivityArgs {
    pub project: String,
}

#[derive(Subcommand)]
pub enum UpdateCommand {
    Activity(UpdateActivityArgs),
    #[command(alias = "at")]
    ActivityType(UpdateActivityTypeArgs),
    Repo(UpdateRepoArgs),
}

#[derive(Args)]
pub struct UpdateActivityArgs {
    #[clap(subcommand)]
    pub command: UpdateActivityCommand,
}

#[derive(Subcommand)]
pub enum UpdateActivityCommand {
    End(UpdateActivityEndArgs),
    #[command(alias = "at")]
    ActivityType(UpdateActivityActivityTypeArgs),
    Description(UpdateActivityDescriptionArgs),
    Project(UpdateActivityProjectArgs),
}

#[derive(Args)]
pub struct UpdateActivityDescriptionArgs {
    pub id: u64,
    pub description: Option<String>,
}
#[derive(Args)]
pub struct UpdateActivityEndArgs {
    pub id: u64,
    #[arg(value_parser = parse_datetime)]
    pub end: DateTime<Utc>,
}

#[derive(Args)]
pub struct UpdateActivityActivityTypeArgs {
    pub id: u64,
    pub atype: String,
}

#[derive(Args)]
pub struct UpdateActivityProjectArgs {
    pub id: u64,
    pub project: String,
}

#[derive(Args)]
pub struct UpdateActivityTypeArgs {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Args)]
pub struct UpdateRepoArgs {
    pub old_path: PathBuf,
    pub new_path: PathBuf,
}
