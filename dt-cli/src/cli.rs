use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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
    ActivityType(AddActivityTypeArgs),
}

#[derive(Args)]
pub struct AddProjectArgs {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Args)]
pub struct AddActivityTypeArgs {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Subcommand)]
pub enum CancelCommand {
    Activity,
}

#[derive(Subcommand)]
pub enum DeleteCommand {
    Project(DeleteProjectArgs),
    Activity(DeleteActivityArgs),
    ActivityType(DeleteActivityTypeArgs),
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
    Projects,
    Activities(ListActivityArgs),
    ActivityTypes(ListActivityTypeArgs),
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

#[derive(Subcommand)]
pub enum RenameCommand {
    Project(RenameProjectArgs),
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
}

#[derive(Subcommand)]
pub enum StopCommand {
    Activity,
}

#[derive(Subcommand)]
pub enum UpdateCommand {
    Project(UpdateProjectArgs),
    ActivityType(UpdateActivityTypeArgs),
}

#[derive(Args)]
pub struct UpdateProjectArgs {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Args)]
pub struct UpdateActivityTypeArgs {
    pub name: String,
    pub description: Option<String>,
}
