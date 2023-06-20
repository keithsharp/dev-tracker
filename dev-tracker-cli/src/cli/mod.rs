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
    /// The data file to use for tracking projects.  Will be created if it
    /// doesn't exist.
    #[clap(long, env = "DT_DATA_FILE")]
    pub data_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a project, activity type, or repository to the tracker.
    #[clap(subcommand)]
    Add(AddCommand),
    #[clap(subcommand)]
    /// Cancel a running activity.
    Cancel(CancelCommand),
    /// Counts the lines of code in PROJECT. Returns an error is there is
    /// no such project.
    Count(CountCommandArgs),
    #[clap(subcommand)]
    /// Delete a project, activity, activity type, count, or repository from
    /// the tracker.
    Delete(DeleteCommand),
    /// Describe a project, activity, or count.
    #[clap(subcommand)]
    Describe(DescribeCommand),
    /// Generate a text or JSON report for a project
    #[clap(subcommand)]
    Generate(GenerateCommand),
    /// List the projects, activities, activity types, counts, or repositories
    /// in the tracker.
    #[clap(subcommand)]
    List(ListCommand),
    /// Rename a project or activity type.
    #[clap(subcommand)]
    Rename(RenameCommand),
    /// Show the status
    Status(StatusArgs),
    // #[clap(subcommand)]
    // Set(),
    /// Start recording an activity.
    #[clap(subcommand)]
    Start(StartCommand),
    /// Stop recording an activity.
    #[clap(subcommand)]
    Stop(StopCommand),
    /// Update an activity, activity type, or repository.
    #[clap(subcommand)]
    Update(UpdateCommand),
}

#[derive(Subcommand)]
pub enum AddCommand {
    /// Creates a new project with NAME and optionally creates a repository for
    /// the project at PATH. Repositories can also be added to a project using
    /// the 'dt add repo <PROJECT> <PATH>' command. Returns an error if there
    /// is an existing project with the same name.
    Project(AddProjectArgs),
    /// Creates a new activity type with NAME and and optional DESCRPTION.
    /// Returns an error if there is an existing activity type with the same
    /// name.
    #[command(alias = "at")]
    ActivityType(AddActivityTypeArgs),
    /// Creates a new repository with PATH and associates it with PROJECT.
    /// Returns an error is there is no such project or there is an existing
    /// repository with the same path.
    Repo(AddRepoArgs),
}

#[derive(Args)]
pub struct AddProjectArgs {
    /// The name to use for the project, must be unique.
    pub name: String,
    /// An optional path for the projects source code, must be unique.
    pub path: Option<PathBuf>,
}

#[derive(Args)]
pub struct AddActivityTypeArgs {
    /// The name to use for the activity type, must be unique.
    pub name: String,
    /// An optional description for the activity type.
    pub description: Option<String>,
}

#[derive(Args)]
pub struct AddRepoArgs {
    /// The name of the project to associate with this repository, the project
    /// must already exist.
    pub project: String,
    /// The path for the projects source code, must be unique.
    pub path: PathBuf,
}

#[derive(Args)]
pub struct CountCommandArgs {
    /// The project that will have it's lines of code counted.
    pub project: String,
}

#[derive(Subcommand)]
pub enum CancelCommand {
    /// Cancels the running activity for PROJECT. Returns an error if there is
    /// no such project or no running activity on the project.
    Activity(CancelActivityTypeArgs),
}

#[derive(Args)]
pub struct CancelActivityTypeArgs {
    /// A project with a running activity to cancel.
    pub project: String,
}

#[derive(Subcommand)]
pub enum DeleteCommand {
    /// Deletes the project with NAME and all of it's associated activities,
    /// repositories, and counts. Returns an error if there is no such project.
    Project(DeleteProjectArgs),
    /// Deletes the activity with ID. Returns an error if there is no such
    /// activity. To obtain the ID of an activity use the '-v' flag for the 'dt
    /// list activities <PROJECT>' command. Returns an error if there is no such
    /// activity.
    Activity(DeleteActivityArgs),
    /// Deletes the activity type with NAME. Returns an error if there is no
    /// such activity type.
    #[command(alias = "at")]
    ActivityType(DeleteActivityTypeArgs),
    /// Deletes the count with ID. To obtain the ID of a count use the '-v' flag
    /// for the 'dt list counts <PROJECT>' command. Returns an error if there is
    /// no such count.
    Count(DeleteCountArgs),
    /// Deletes the repository with PATH and all of it's associated counts.
    /// Returns an error if there is no such repository.
    Repo(DeleteRepoArgs),
}

#[derive(Args)]
pub struct DeleteProjectArgs {
    /// The project to delete.
    pub name: String,
}

#[derive(Args)]
pub struct DeleteActivityArgs {
    /// The activity to delete.
    pub id: u64,
}

#[derive(Args)]
pub struct DeleteActivityTypeArgs {
    /// The activity type to delete/
    pub name: String,
}

#[derive(Args)]
pub struct DeleteCountArgs {
    /// The count to delete.
    pub id: u64,
}

#[derive(Args)]
pub struct DeleteRepoArgs {
    /// The repository to delete.  Only deletes from the tracker, does not
    /// delete your files!
    pub path: PathBuf,
}

#[derive(Subcommand)]
pub enum DescribeCommand {
    /// Describes the project with NAME. Lists the repositories, a count of
    /// the activities for the project, and a count of the total lines of code
    /// for the project. Returns an error if there is no such project.
    Project(DescribeProjectArgs),
    /// Describes the activity with ID. Lists the project name, the activity
    /// type, the start and end times (or still running), and the duration in
    /// minutes. To obtain the ID of an activity use the '-v' flag for the 'dt
    /// list activities <PROJECT>' command. Returns an error if there is no
    /// such activity.
    Activity(DescribeActivityArgs),
    /// Describes the count with ID. Lists project name, the repository path,
    /// the date and time of the count, and the lines of code. To obtain
    /// the ID of a count use the '-v' flag for the 'dt list counts <PROJECT>'
    /// command. Returns an error if there is no such count, project, or
    /// repository.
    Count(DescribeCountArgs),
}

#[derive(Args)]
pub struct DescribeProjectArgs {
    /// The project to describe.
    pub name: String,
}

#[derive(Args)]
pub struct DescribeActivityArgs {
    /// The activity to describe.
    pub id: u64,
}

#[derive(Args)]
pub struct DescribeCountArgs {
    /// The count to describe.
    pub id: u64,
}

#[derive(Subcommand)]
pub enum GenerateCommand {
    /// Generate a text report for the project.
    Report(GenerateArgs),
    /// Generate a JSON report for the project.
    Json(GenerateArgs),
}

#[derive(Args)]
pub struct GenerateArgs {
    /// The name of the project to report on.  Use 'all' to report on all
    /// projects.  
    pub name: String,
    /// An optional start date for the report.  The date format is DD-MM-YYYY.
    /// If omitted the report uses all activites up until the end date or now.
    #[arg(value_parser = parse_date)]
    pub start: Option<DateTime<Utc>>,
    /// An optional end date for the report.  The date format is DD-MM-YYYY.
    /// If omitted the report uses an end date of now.
    #[arg(value_parser = parse_date)]
    pub end: Option<DateTime<Utc>>,
}

#[derive(Subcommand)]
pub enum ListCommand {
    /// List all of the projects in the database. Use the optional '-v' flag
    /// to list the ID numbers for the projects.
    Projects(ListProjectArgs),
    /// List all of the activities for PROJECT. Use the optional '-v' flag to
    /// list the ID numbers for the activities. Returns an error if there is
    /// no such project.
    Activities(ListActivityArgs),
    /// List all of the activity types. Use the optional '-v' flag to list the
    /// ID numbers for the activity types.
    #[command(alias = "ats")]
    ActivityTypes(ListActivityTypeArgs),
    /// List all of the counts for PROJECT. Use the optional '-v' flag to list
    /// the ID numbers for the counts. Returns an error if there is no such
    /// project.
    Counts(ListCountArgs),
    /// List all of the repositories for PROJECT. Use the optional '-v' flag to
    /// list the ID numbers for the repositories. Returns an error if there
    /// is no such project.
    Repos(ListRepoArgs),
}

#[derive(Args)]
pub struct ListProjectArgs {
    /// List the ID numbers.
    #[clap(short, action)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct ListActivityArgs {
    /// List the ID numbers.
    #[clap(short, action)]
    pub verbose: bool,
    /// The project with activities to list.
    pub project: String,
}

#[derive(Args)]
pub struct ListActivityTypeArgs {
    /// List the ID numbers.
    #[clap(short, action)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct ListCountArgs {
    /// List the ID numbers.
    #[clap(short, action)]
    pub verbose: bool,
    /// The project with counts to list.
    pub project: String,
}

#[derive(Args)]
pub struct ListRepoArgs {
    /// List the ID numbers.
    #[clap(short, action)]
    pub verbose: bool,
    /// The project with repositories to list.
    pub project: String,
}

#[derive(Subcommand)]
pub enum RenameCommand {
    /// Renames a project from OLD_NAME to NEW_NAME. Returns an error if there
    /// is no project with name NEW_NAME or if there is an existing project
    /// with NEW_NAME.
    Project(RenameProjectArgs),
    /// Renames an activity type from OLD_NAME to NEW_NAME. Returns an error
    /// if there is no activity type with name NEW_NAME or if there is an
    /// existing activity type with NEW_NAME.
    #[command(alias = "at")]
    ActivityType(RenameActivityTypeArgs),
}

#[derive(Args)]
pub struct RenameProjectArgs {
    /// The current project name.
    pub old_name: String,
    /// The new project name.
    pub new_name: String,
}

#[derive(Args)]
pub struct RenameActivityTypeArgs {
    /// The current activity type name.
    pub old_name: String,
    /// The new activity type name.
    pub new_name: String,
}

#[derive(Args)]
pub struct StatusArgs {
    /// The name of the project
    pub name: Option<String>,
}

#[derive(Subcommand)]
pub enum StartCommand {
    /// Start recording an activity for PROJECT with an activity type of
    /// ACTIVITY_TYPE and an optional DESCRIPTION. Returns an error if
    /// there is no such project or activity type, or if there already an
    /// activity in progress for the project.
    Activity(StartActivityArgs),
}

#[derive(Args)]
pub struct StartActivityArgs {
    /// The project on which to start recording an activity.
    pub project: String,
    /// The type of the activity.
    pub activity_type: String,
    /// An optional description of the activity.
    pub description: Option<String>,
}

#[derive(Subcommand)]
pub enum StopCommand {
    /// Stops the current activity for PROJECT. Returns an error if there is
    /// no such project or of there is no current activity in progress.
    Activity(StopActivityArgs),
}

#[derive(Args)]
pub struct StopActivityArgs {
    /// The project which has a running activity to stop.
    pub project: String,
    /// An optional description of the activity, will overwrite any
    /// description that already exists.
    pub description: Option<String>,
    /// Override the default behavior of creating a count of the lines of code
    /// in a project at the end of the activity.
    #[clap(long, action)]
    pub no_count: bool,
}

#[derive(Subcommand)]
pub enum UpdateCommand {
    /// Update the values associated with an activity.
    Activity(UpdateActivityArgs),
    /// Update the description of an actovoty type.
    #[command(alias = "at")]
    ActivityType(UpdateActivityTypeArgs),
    /// Update the path of a repository.
    Repo(UpdateRepoArgs),
}

#[derive(Args)]
pub struct UpdateActivityArgs {
    #[clap(subcommand)]
    pub command: UpdateActivityCommand,
}

#[derive(Subcommand)]
pub enum UpdateActivityCommand {
    /// Updates the end time for the activity with ID.  END should be in the
    /// format YYYY-MM-DDTHH:MM where the date and time are in your local
    /// timezone and the time uses the 24-hour clock. To obtain the ID of an
    /// activity use the '-v' flag for the 'dt list activities <PROJECT>'
    /// command. Returns an error if there is no such activity, if the new
    /// end time is before the activity start time, or if there is an error
    /// parsing the date and time from your local timezone into UTC.
    End(UpdateActivityEndArgs),
    /// Updates the activity type for the activity with ID. Returns an error if
    /// there is no such activity or no such activity type.
    #[command(alias = "at")]
    ActivityType(UpdateActivityActivityTypeArgs),
    /// Updates the description for the activity with ID. Use this command
    /// with no value for DESCRIPTION to remove an existing description. To
    /// obtain the ID of an activity use the '-v' flag for the 'dt list
    /// activities <PROJECT>' command. Returns an error if there is no such
    /// activity.
    Description(UpdateActivityDescriptionArgs),
    /// Updates the project for the activity with ID. To obtain the ID of an
    /// activity use the '-v' flag for the 'dt list activities <PROJECT>'
    /// command. Returns an error if there is no such activity or no such
    /// project.
    Project(UpdateActivityProjectArgs),
}

#[derive(Args)]
pub struct UpdateActivityDescriptionArgs {
    /// The activity to update.
    pub id: u64,
    /// The new description, leave blank to remove an existing description.
    pub description: Option<String>,
}
#[derive(Args)]
pub struct UpdateActivityEndArgs {
    /// The activity to update.
    pub id: u64,
    /// The new end date and time.  Must be in the
    /// format YYYY-MM-DDTHH:MM where the date and time are in your local
    /// timezone and the time uses the 24-hour clock.
    #[arg(value_parser = parse_datetime)]
    pub end: DateTime<Utc>,
}

#[derive(Args)]
pub struct UpdateActivityActivityTypeArgs {
    /// The activity to update.
    pub id: u64,
    /// The new activity type.
    pub atype: String,
}

#[derive(Args)]
pub struct UpdateActivityProjectArgs {
    /// The activity to update.
    pub id: u64,
    /// The new project.
    pub project: String,
}

#[derive(Args)]
pub struct UpdateActivityTypeArgs {
    /// The activity type to update.
    pub name: String,
    /// The new description, leave blank to remove an existing description.
    pub description: Option<String>,
}

#[derive(Args)]
pub struct UpdateRepoArgs {
    /// The current repository path.
    pub old_path: PathBuf,
    /// The new repository path.
    pub new_path: PathBuf,
}
