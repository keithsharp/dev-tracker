use std::process;

use chrono::{DateTime, Local};

use dev_tracker_core::model::{Activity, Project};
use dev_tracker_core::{data::DataStore, model::ActivityType};

use crate::cli::{
    AddActivityTypeArgs, AddProjectArgs, DeleteActivityArgs, DeleteActivityTypeArgs,
    DeleteProjectArgs, DescribeActivityArgs, DescribeProjectArgs, ListActivityArgs,
    ListActivityTypeArgs, RenameActivityTypeArgs, RenameProjectArgs, StartActivityArgs,
    UpdateActivityTypeArgs, UpdateProjectArgs,
};

pub fn add_project(args: AddProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_project_with_name(&args.name)? {
        Some(_) => {
            eprintln!("Add failed, project already exists: {}", args.name);
            process::exit(1);
        }
        None => {
            let project = Project::new(args.name, args.path);
            ds.add_project(&project)?;
        }
    }

    Ok(())
}

pub fn add_activitytype(args: AddActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_activitytype_with_name(&args.name)? {
        Some(_) => {
            eprintln!("Add failed, activity type already exists: {}", args.name);
            process::exit(1);
        }
        None => {
            let at = ActivityType::new(args.name, args.description);
            ds.add_activitytype(at)?;
        }
    }

    Ok(())
}

pub fn delete_project(args: DeleteProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_project_with_name(&args.name)? {
        Some(project) => ds.delete_project(project)?,
        None => {
            eprintln!("Delete failed, no such project: {}", args.name);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn delete_activity(args: DeleteActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_activity_with_id(args.id)? {
        Some(activity) => ds.delete_activity(activity)?,
        None => {
            eprintln!("Delete failed, no such activity: {}", args.id);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn delete_activitytype(args: DeleteActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_activitytype_with_name(&args.name)? {
        Some(at) => ds.delete_activitytype(at)?,
        None => {
            eprintln!("Delete failed, no such activity type: {}", args.name);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn describe_project(args: DescribeProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_project_with_name(&args.name)? {
        Some(project) => {
            println!("Project name: {}", project.name());
            println!("Repository path: {}", project.path().display());
        }
        None => {
            eprintln!("Describe failed, no such project: {}", args.name);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn describe_activity(args: DescribeActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let activity = match ds.get_activity_with_id(args.id)? {
        Some(activity) => activity,
        None => {
            eprintln!("Describe failed, no such activity: {}", args.id);
            process::exit(1);
        }
    };

    let at = match ds.get_activitytype_with_id(activity.atype())? {
        Some(at) => at,
        None => {
            eprintln!(
                "Describe failed, no activity type for activity: {}",
                args.id
            );
            process::exit(1);
        }
    };

    let project = match ds.get_project_with_id(activity.project())? {
        Some(project) => project,
        None => {
            eprintln!("Describe failed, no project for activity: {}", args.id);
            process::exit(1);
        }
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

pub fn list_projects(ds: &DataStore) -> anyhow::Result<()> {
    let projects = ds.get_projects()?;
    for project in projects.iter() {
        println!("{}", project);
    }

    if projects.len() < 1 {
        println!("No projects in database");
    }

    Ok(())
}

pub fn list_activities(args: ListActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    let project = match ds.get_project_with_name(&args.project)? {
        Some(project) => project,
        None => {
            eprintln!("List activities failed, no such project: {}", args.project);
            process::exit(1);
        }
    };

    let activities = ds.get_activities(&project)?;
    for activity in activities.iter() {
        let at = ds
            .get_activitytype_with_id(activity.atype())?
            .expect("should always be able to get an activity type");
        if args.verbose {
            print!("{}. ", activity.id());
        }
        println!("{} {}", at, activity);
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
        println!("{}", at);
    }

    if ats.len() < 1 {
        println!("No activity types in database");
    }

    Ok(())
}

pub fn rename_project(args: RenameProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_project_with_name(&args.old_name)? {
        Some(mut project) => {
            project.set_name(args.new_name);
            ds.update_project(&project)?;
        }
        None => {
            eprintln!("Rename failed, no such project: {}", args.old_name);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn rename_activitytype(args: RenameActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_activitytype_with_name(&args.old_name)? {
        Some(mut at) => {
            at.set_name(args.new_name);
            ds.update_activitytype(&at)?;
        }
        None => {
            eprintln!("Rename failed, no such activity type: {}", args.old_name);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn start_activity(args: StartActivityArgs, ds: &DataStore) -> anyhow::Result<()> {
    if ds.get_running_activity()?.is_some() {
        eprintln!(
            "Start activity failed, an activity is aready running for project: {}",
            args.project
        );
        process::exit(1);
    }

    let project = match ds.get_project_with_name(&args.project)? {
        Some(project) => project,
        None => {
            eprintln!("Start activity failed, no such project: {}", args.project);
            process::exit(1);
        }
    };

    let at = match ds.get_activitytype_with_name(&args.activity_type)? {
        Some(at) => at,
        None => {
            eprintln!(
                "Start activity failed, no such activity type: {}",
                args.activity_type
            );
            process::exit(1);
        }
    };

    let activity = Activity::new(&project, at);
    ds.start_activity(activity)?;

    Ok(())
}

pub fn stop_activity(ds: &DataStore) -> anyhow::Result<()> {
    let Some(_activity) = ds.stop_running_activity()? else {
        eprintln!("Stop activity failed, no activity running");
        process::exit(1)
    };
    Ok(())
}

pub fn update_project(args: UpdateProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_project_with_name(&args.name)? {
        Some(mut project) => {
            project.set_path(args.path);
            ds.update_project(&project)?;
        }
        None => {
            eprintln!("Update failed, no such project: {}", args.name);
            process::exit(1);
        }
    }

    Ok(())
}

pub fn update_activitytype(args: UpdateActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    match ds.get_activitytype_with_name(&args.name)? {
        Some(mut at) => {
            at.set_description(args.description);
            ds.update_activitytype(&at)?;
        }
        None => {
            eprintln!("Update failed, no such activity type: {}", args.name);
            process::exit(1);
        }
    }

    Ok(())
}
