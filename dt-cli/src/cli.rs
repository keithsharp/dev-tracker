use std::path::PathBuf;

use chrono::offset::TimeZone;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long)]
    pub config_file: Option<PathBuf>,
}

fn parse_datetime(arg: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    let datetime = NaiveDateTime::parse_from_str(arg, "%Y-%m-%dT%H:%M")?;
    let datetime = match Local.from_local_datetime(&datetime) {
        chrono::LocalResult::Single(datetime) => datetime,
        _ => panic!("Failed to convert local datetime into UTC datetime."),
    };
    let datetime: DateTime<Utc> = DateTime::from(datetime);

    Ok(datetime)
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
    pub description: Option<String>,
}

#[derive(Subcommand)]
pub enum StopCommand {
    Activity,
}

#[derive(Subcommand)]
pub enum UpdateCommand {
    Activity(UpdateActivityArgs),
    ActivityType(UpdateActivityTypeArgs),
}

#[derive(Args)]
pub struct UpdateActivityArgs {
    #[clap(subcommand)]
    pub command: UpdateActivityCommand,
}

#[derive(Subcommand)]
pub enum UpdateActivityCommand {
    End(UpdateActivityEndArgs),
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
