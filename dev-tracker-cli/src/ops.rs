use std::process;

use chrono::{DateTime, Local};

use dev_tracker_core::data::DataStore;

use crate::cli::{
    AddActivityTypeArgs, AddProjectArgs, AddRepoArgs, CancelActivityTypeArgs, CountCommandArgs,
    DeleteActivityArgs, DeleteActivityTypeArgs, DeleteCountArgs, DeleteProjectArgs, DeleteRepoArgs,
    DescribeActivityArgs, DescribeCountArgs, DescribeProjectArgs, ListActivityArgs,
    ListActivityTypeArgs, ListCountArgs, ListProjectArgs, ListRepoArgs, RenameActivityTypeArgs,
    RenameProjectArgs, StartActivityArgs, StopActivityArgs, UpdateActivityActivityTypeArgs,
    UpdateActivityDescriptionArgs, UpdateActivityEndArgs, UpdateActivityProjectArgs,
    UpdateActivityTypeArgs, UpdateRepoArgs,
};

pub fn add_project(args: AddProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    ds.create_project(&args.name)?;

    if let Some(path) = args.path {
        let project = ds
            .get_project(&args.name)?
            .expect("should always be able to get the project we just created");
        ds.create_repo(&project, &path)?;
    }

    Ok(())
}

pub fn add_activitytype(args: AddActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    ds.create_activitytype(&args.name, args.description)?;

    Ok(())
}

pub fn add_repo(args: AddRepoArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("Add failed, no such project: {}", args.project);
        process::exit(1);
    };

    ds.create_repo(&project, &args.path)?;

    Ok(())
}

pub fn count(args: CountCommandArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("Cancel failed, no such project: {}", args.project);
        process::exit(1);
    };

    let repos = ds.get_repos(&project)?;

    for repo in repos.iter() {
        let _count = ds.create_count(repo)?;
    }

    Ok(())
}

pub fn cancel_actvity(args: CancelActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("Cancel failed, no such project: {}", args.project);
        process::exit(1);
    };

    ds.cancel_running_actvity(&project)?;

    Ok(())
}

pub fn delete_project(args: DeleteProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.name)? else {
        eprintln!("Delete failed, no such project: {}", args.name);
        process::exit(1);
    };

    ds.delete_project(project)?;

    Ok(())
}

pub fn delete_activity(args: DeleteActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(activity) = ds.get_activity_with_id(args.id)? else {
        eprintln!("Delete failed, no such activity: {}", args.id);
        process::exit(1);
    };

    ds.delete_activity(activity)?;

    Ok(())
}

pub fn delete_activitytype(args: DeleteActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(at) = ds.get_activitytype(&args.name)? else {
        eprintln!("Delete failed, no such activity type: {}", args.name);
        process::exit(1);
    };

    ds.delete_activitytype(at)?;

    Ok(())
}

pub fn delete_count(args: DeleteCountArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(count) = ds.get_count_with_id(args.id)? else {
        eprintln!("Delete failed, no such count: {}", args.id);
        process::exit(1);
    };

    ds.delete_count(count)?;

    Ok(())
}

pub fn delete_repo(args: DeleteRepoArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(repo) = ds.get_repo(&args.path)? else {
        eprintln!("Delete failed, no such repository with path: {}", args.path.display());
        process::exit(1);
    };

    ds.delete_repo(repo)?;

    Ok(())
}

pub fn describe_project(args: DescribeProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.name)? else {
        eprintln!("Describe failed, no such project: {}", args.name);
        process::exit(1);
    };

    println!("Project name '{}'", project.name());

    let repos = ds.get_repos(&project)?;
    for repo in repos.iter() {
        println!("Repository path '{}'", repo.path().display());
        if let Some(count) = ds.get_latest_count(&repo)? {
            println!("  {} lines of code", count.count())
        }
    }
    if repos.is_empty() {
        println!("No repositories")
    }

    let activities = ds.get_activities(&project)?;
    if activities.is_empty() {
        println!("No activities")
    } else {
        println!("Total activity count {}", activities.len());
    }

    let ats = ds.get_activitytypes()?;
    if !ats.is_empty() && !activities.is_empty() {
        for at in ats.iter() {
            let count = activities.iter().filter(|a| a.atype() == at.id()).count();
            if count > 0 {
                print!("Activity type '{}' has {} ", at.name(), count);
                if count == 1 {
                    println!("activity");
                } else {
                    println!("activities");
                }
            }
        }
    }

    Ok(())
}

pub fn describe_activity(args: DescribeActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(activity) = ds.get_activity_with_id(args.id)? else {
        eprintln!("Describe failed, no such activity: {}", args.id);
        process::exit(1);
    };

    let Some(at) = ds.get_activitytype_with_id(activity.atype())? else {
        eprintln!(
            "Describe failed, no activity type for activity: {}",
            args.id
        );
        process::exit(1);
    };

    let Some(project) = ds.get_project_with_id(activity.project())? else {
        eprintln!("Describe failed, no project for activity: {}", args.id);
        process::exit(1);
    };

    println!("Project: {}", project.name());
    println!("Activity type: {}", at.name());

    let local_start: DateTime<Local> = DateTime::from(activity.start_time());
    println!("Started: {}", local_start.format("%I:%M%P on %A %d %B %Y"));

    if let Some(end) = activity.end_time() {
        let local_end: DateTime<Local> = DateTime::from(end);
        println!("Finished: {}", local_end.format("%I:%M%P on %A %d %B %Y"));
        let minutes = activity
            .duration()
            .expect("we have an end so we should have a duration")
            .num_minutes();
        print!("Duration: {} ", minutes);
        if minutes == 1 {
            println!("minute");
        } else {
            println!("minutes");
        }
    } else {
        println!("Finished: still running");
    }

    Ok(())
}

pub fn describe_count(args: DescribeCountArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(count) = ds.get_count_with_id(args.id)? else {
        eprintln!("Describe failed, no such count: {}", args.id);
        process::exit(1);
    };

    let Some(repo) = ds.get_repo_with_id(count.repo())? else {
        eprintln!("Describe failed, no such repository: {}", count.repo());
        process::exit(1);
    };

    let Some(project) = ds.get_project_with_id(repo.project())? else {
        eprintln!("Describe failed, no such project: {}", repo.project());
        process::exit(1);
    };

    println!("Project: {}", project.name());
    println!("Repository: {}", repo.path().display());
    println!("Date: {}", count.date().format("%A %d %B %Y at %I:%M%P"));
    println!("Lines of code: {}", count.count());

    Ok(())
}

pub fn list_projects(args: ListProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    let projects = ds.get_projects()?;
    for project in projects.iter() {
        if args.verbose {
            print!("{}. ", project.id());
        }
        println!("{}", project.name());
    }

    if projects.len() < 1 {
        println!("No projects in database");
    }

    Ok(())
}

pub fn list_activities(args: ListActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("List activities failed, no such project: {}", args.project);
        process::exit(1);
    };

    let activities = ds.get_activities(&project)?;
    for activity in activities.iter() {
        let at = ds
            .get_activitytype_with_id(activity.atype())?
            .expect("should always be able to get an activity type");

        if args.verbose {
            print!("{}. ", activity.id());
        }
        print!("{} ", at.name());

        let local_start: DateTime<Local> = DateTime::from(activity.start_time());
        if let Some(end) = activity.end_time() {
            let local_end: DateTime<Local> = DateTime::from(end);
            let dur = activity
                .duration()
                .expect("we have an end so we should have a duration");
            let hours = dur.num_hours();
            let minutes = dur.num_minutes() % 60;

            let hours = match hours {
                0 => String::new(),
                1 => "1 hour".to_string(),
                h => format!("{} hours", h),
            };

            let minutes = match minutes {
                0 => String::new(),
                1 => "1 minute".to_string(),
                m => format!("{} minutes", m),
            };

            let duration = match (hours.is_empty(), minutes.is_empty()) {
                (false, false) => format!("{} {}", hours, minutes),
                (true, false) => minutes,
                (false, true) => hours,
                (true, true) => "less than a minutes".to_string(),
            };

            println!(
                "from {} until {}, total time {}",
                local_start.format("%I:%M%P on %A %d %B %Y").to_string(),
                local_end.format("%I:%M%P on %A %d %B %Y").to_string(),
                duration
            )
        } else {
            println!(
                "started at {}, and is still running",
                local_start.format("%I:%M%P on %A %d %B %Y").to_string()
            )
        }
    }

    if activities.len() < 1 {
        println!("No activities for project {} in database", project.name());
    }

    Ok(())
}

pub fn list_activitytypes(args: ListActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let ats = ds.get_activitytypes()?;
    for at in ats.iter() {
        if args.verbose {
            print!("{}. ", at.id());
        }
        println!("{}", at.name());
    }

    if ats.len() < 1 {
        println!("No activity types in database");
    }

    Ok(())
}

pub fn list_counts(args: ListCountArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("List counts failed, no such project: {}", args.project);
        process::exit(1);
    };

    let repos = ds.get_repos(&project)?;
    for repo in repos.iter() {
        let counts = ds.get_counts(&repo)?;
        for count in counts {
            if args.verbose {
                print!("{}. ", count.id());
            }
            println!(
                "{} {} has {} lines of code",
                count.date().format("%A %d %B %Y at %I:%M%P"),
                repo.path().display(),
                count.count()
            );
        }
    }

    if repos.len() < 1 {
        println!("No repositories for project {} in database", project.name());
    }

    Ok(())
}

pub fn list_repos(args: ListRepoArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("List repositories failed, no such project: {}", args.project);
        process::exit(1);
    };

    let repos = ds.get_repos(&project)?;
    for repo in repos.iter() {
        if args.verbose {
            print!("{}. ", repo.id());
        }
        println!("{}", repo.path().display());
    }

    if repos.len() < 1 {
        println!("No repositories for project {} in database", project.name());
    }

    Ok(())
}

pub fn rename_project(args: RenameProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(mut project) = ds.get_project(&args.old_name)? else {
        eprintln!("Rename failed, no such project: {}", args.old_name);
        process::exit(1);
    };

    if ds.get_project(&args.new_name)?.is_some() {
        eprintln!("Rename failed, project already exists: {}", args.new_name);
        process::exit(1);
    }

    project.set_name(args.new_name);
    ds.update_project(&project)?;

    Ok(())
}

pub fn rename_activitytype(args: RenameActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(mut at) = ds.get_activitytype(&args.old_name)? else {
        eprintln!("Rename failed, no such activity type: {}", args.old_name);
        process::exit(1);
    };

    if ds.get_activitytype(&args.new_name)?.is_some() {
        eprintln!(
            "Rename failed, activity type already exists: {}",
            args.new_name
        );
        process::exit(1);
    }

    at.set_name(args.new_name);
    ds.update_activitytype(&at)?;

    Ok(())
}

pub fn start_activity(args: StartActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("Start activity failed, no such project: {}", args.project);
        process::exit(1);
    };

    if ds.get_running_activity(&project)?.is_some() {
        eprintln!(
            "Start activity failed, an activity is aready running for project: {}",
            project.name()
        );
        process::exit(1);
    }

    let Some(at) = ds.get_activitytype(&args.activity_type)? else {
        eprintln!(
            "Start activity failed, no such activity type: {}",
            args.activity_type
        );
        process::exit(1);
    };

    ds.start_activity(&project, &at, args.description)?;

    Ok(())
}

pub fn stop_activity(args: StopActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("Stop activity failed, no such project: {}", args.project);
        process::exit(1);
    };

    if ds.stop_running_activity(&project)?.is_none() {
        eprintln!("Stop activity failed, no activity running");
        process::exit(1)
    }

    Ok(())
}

pub fn update_activity_description(
    args: UpdateActivityDescriptionArgs,
    ds: &DataStore,
) -> anyhow::Result<()> {
    let Some(mut activity) = ds.get_activity_with_id(args.id)? else {
        eprintln!("Update failed, no such actvity: {}", args.id);
        process::exit(1);
    };

    activity.set_description(args.description);
    ds.update_activity(&activity)?;

    Ok(())
}

pub fn update_activity_atype(
    args: UpdateActivityActivityTypeArgs,
    ds: &DataStore,
) -> anyhow::Result<()> {
    let Some(mut activity) = ds.get_activity_with_id(args.id)? else {
        eprintln!("Update failed, no such actvity: {}", args.id);
        process::exit(1);
    };

    let Some(at) = ds.get_activitytype(&args.atype)? else {
        eprintln!("Update failed, no such activity type: {}", args.atype);
        process::exit(1);
    };

    activity.set_atype(at.id());
    ds.update_activity(&activity)?;

    Ok(())
}

pub fn update_activity_end(args: UpdateActivityEndArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(mut activity) = ds.get_activity_with_id(args.id)? else {
        eprintln!("Update failed, no such actvity: {}", args.id);
        process::exit(1);
    };

    if args.end < activity.start_time() {
        let local_start: DateTime<Local> = DateTime::from(activity.start_time());
        eprintln!(
            "Update failed, end time {} is before start time {}",
            args.end.format("%I:%M%P on %A %d %B %Y"),
            local_start.format("%I:%M%P on %A %d %B %Y")
        );
        process::exit(1);
    }

    activity.set_end_time(Some(args.end));
    ds.update_activity(&activity)?;

    Ok(())
}

pub fn update_activity_project(
    args: UpdateActivityProjectArgs,
    ds: &DataStore,
) -> anyhow::Result<()> {
    let Some(mut activity) = ds.get_activity_with_id(args.id)? else {
        eprintln!("Update failed, no such actvity: {}", args.id);
        process::exit(1);
    };

    let Some(project) = ds.get_project(&args.project)? else {
        eprintln!("Update failed, no such project: {}", args.project);
        process::exit(1);
    };

    activity.set_project(project.id());
    ds.update_activity(&activity)?;

    Ok(())
}

pub fn update_activitytype(args: UpdateActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(mut at) = ds.get_activitytype(&args.name)? else {
        eprintln!("Update failed, no such activity type: {}", args.name);
        process::exit(1);
    };

    at.set_description(args.description);
    ds.update_activitytype(&at)?;

    Ok(())
}

pub fn update_repo(args: UpdateRepoArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(mut repo) = ds.get_repo(&args.old_path)? else {
        eprintln!("Update failed, no such repository with path {}", args.old_path.display());
        process::exit(1);
    };

    if ds.get_repo(&args.new_path)?.is_some() {
        eprintln!(
            "Update failed, repository with path already exists: {}",
            args.new_path.display()
        );
        process::exit(1);
    }

    repo.set_path(args.new_path);
    ds.update_repo(&repo)?;

    Ok(())
}
