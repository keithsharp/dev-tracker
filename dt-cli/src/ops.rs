use std::process;

use chrono::{DateTime, Local};

use dev_tracker_core::data::DataStore;

use crate::cli::{
    AddActivityTypeArgs, AddProjectArgs, CancelActivityTypeArgs, DeleteActivityArgs,
    DeleteActivityTypeArgs, DeleteProjectArgs, DescribeActivityArgs, DescribeProjectArgs,
    ListActivityArgs, ListActivityTypeArgs, RenameActivityTypeArgs, RenameProjectArgs,
    StartActivityArgs, StopActivityArgs, UpdateActivityActivityTypeArgs,
    UpdateActivityDescriptionArgs, UpdateActivityEndArgs, UpdateActivityProjectArgs,
    UpdateActivityTypeArgs,
};

pub fn add_project(args: AddProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    ds.create_project(&args.name)?;

    Ok(())
}

pub fn add_activitytype(args: AddActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    ds.create_activitytype(&args.name, args.description)?;

    Ok(())
}

pub fn cancel_actvity(args: CancelActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.name)? else {
        eprintln!("Cancel failed, no such project: {}", args.name);
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

pub fn describe_project(args: DescribeProjectArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(project) = ds.get_project(&args.name)? else {
        eprintln!("Describe failed, no such project: {}", args.name);
        process::exit(1);
    };

    println!("Project name: {}", project.name());

    let repos = ds.get_repos(&project)?;
    for repo in repos {
        println!("Repository path: {}", repo.path().display());
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
    let Some(mut project) = ds.get_project(&args.old_name)? else {
        eprintln!("Rename failed, no such project: {}", args.old_name);
        process::exit(1);
    };

    project.set_name(args.new_name);
    ds.update_project(&project)?;

    Ok(())
}

pub fn rename_activitytype(args: RenameActivityTypeArgs, ds: &DataStore) -> anyhow::Result<()> {
    let Some(mut at) = ds.get_activitytype(&args.old_name)? else {
        eprintln!("Rename failed, no such activity type: {}", args.old_name);
        process::exit(1);
    };

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
