use std::process;

use dev_tracker_core::model::Project;
use dev_tracker_core::{data::DataStore, model::ActivityType};

use crate::cli::{
    AddActivityTypeArgs, AddProjectArgs, DeleteActivityTypeArgs, DeleteProjectArgs,
    DescribeProjectArgs, RenameActivityTypeArgs, RenameProjectArgs, UpdateActivityTypeArgs,
    UpdateProjectArgs,
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

pub fn list_activitytypes(ds: &DataStore) -> anyhow::Result<()> {
    let ats = ds.get_activitytypes()?;
    for at in ats.iter() {
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
