use std::path::{Path, PathBuf};

use chrono::Utc;
use rusqlite::Connection;

use crate::model::activity::Activity;
use crate::model::activitytype::ActivityType;
use crate::model::project::Project;
use crate::model::repo::Repo;
use crate::model::{activity, activitytype, project, repo};
use crate::Error;

#[derive(Debug)]
pub struct DataStore {
    conn: Connection,
}

impl DataStore {
    pub fn new(file: Option<&Path>) -> Result<Self, Error> {
        let ds = DataStore::open(file)?;
        ds.init_tables()?;

        Ok(ds)
    }

    pub fn open(file: Option<&Path>) -> Result<Self, Error> {
        let conn = match file {
            Some(file) => Connection::open(file)?,
            None => Connection::open_in_memory()?,
        };

        let ds = Self { conn };
        Ok(ds)
    }

    fn init_tables(&self) -> Result<(), Error> {
        project::init_table(&self.conn)?;
        activity::init_table(&self.conn)?;
        activitytype::init_table(&self.conn)?;
        repo::init_table(&self.conn)?;

        Ok(())
    }
}

// Project
impl DataStore {
    pub fn create_project(&self, name: &str) -> Result<Project, Error> {
        if !Project::get_with_name(name, &self.conn)?.is_empty() {
            return Err(Error::ProjectAlreadyExists(name.to_string()));
        }

        let project = Project::new(name.to_string());
        project.create(&self.conn)?;

        Ok(project)
    }

    pub fn delete_project(&self, project: Project) -> Result<(), Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let activities: Vec<_> = Activity::get_all(&self.conn)?
            .into_iter()
            .filter(|a| a.project == project.id)
            .collect();

        for activity in activities {
            activity.delete(&self.conn)?;
        }

        let repos: Vec<_> = Repo::get_all(&self.conn)?
            .into_iter()
            .filter(|r| r.project == project.id)
            .collect();

        for repo in repos {
            repo.delete(&self.conn)?;
        }

        project.delete(&self.conn)?;

        Ok(())
    }

    pub fn update_project(&self, project: &Project) -> Result<(), Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        project.update(&self.conn)?;

        Ok(())
    }

    pub fn get_project(&self, name: &str) -> Result<Option<Project>, Error> {
        let mut projects = Project::get_with_name(name, &self.conn)?;

        if projects.len() == 1 {
            return Ok(Some(projects.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_project_with_id(&self, id: u64) -> Result<Option<Project>, Error> {
        let project = Project::get_with_id(id, &self.conn)?;
        Ok(project)
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let projects = Project::get_all(&self.conn)?;

        Ok(projects)
    }
}

// ActivityType
impl DataStore {
    pub fn create_activitytype(
        &self,
        name: &str,
        description: Option<String>,
    ) -> Result<ActivityType, Error> {
        if !ActivityType::get_with_name(name, &self.conn)?.is_empty() {
            return Err(Error::ActivityTypeAlreadyExists(name.to_string()));
        }

        let at = ActivityType::new(name.to_string(), description);
        at.create(&self.conn)?;

        Ok(at)
    }

    pub fn delete_activitytype(&self, at: ActivityType) -> Result<(), Error> {
        let Some(at) = ActivityType::get_with_id(at.id, &self.conn)? else {
            return Err(Error::ActivityTypeNotFound(at.id.to_string()));
        };

        let activities: Vec<_> = Activity::get_all(&self.conn)?
            .into_iter()
            .filter(|a| a.atype == at.id)
            .collect();
        if activities.len() > 0 {
            return Err(Error::ActivityTypeInUse(at.id.to_string()));
        }

        at.delete(&self.conn)?;

        Ok(())
    }

    pub fn update_activitytype(&self, at: &ActivityType) -> Result<(), Error> {
        let Some(at) = ActivityType::get_with_id(at.id, &self.conn)? else {
            return Err(Error::ActivityTypeNotFound(at.id.to_string()));
        };

        at.update(&self.conn)?;

        Ok(())
    }

    pub fn get_activitytype(&self, name: &str) -> Result<Option<ActivityType>, Error> {
        let mut ats = ActivityType::get_with_name(name, &self.conn)?;

        if ats.len() == 1 {
            return Ok(Some(ats.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_activitytype_with_id(&self, id: u64) -> Result<Option<ActivityType>, Error> {
        let at = ActivityType::get_with_id(id, &self.conn)?;
        Ok(at)
    }

    pub fn get_activitytypes(&self) -> Result<Vec<ActivityType>, Error> {
        let ats = ActivityType::get_all(&self.conn)?;

        Ok(ats)
    }
}

// Activity
impl DataStore {
    pub fn start_activity(
        &self,
        project: &Project,
        at: &ActivityType,
        description: Option<String>,
    ) -> Result<Activity, Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        if self.get_running_activity(&project)?.is_some() {
            return Err(Error::RunningActivityAlreadyExists(project.id.to_string()));
        }

        let Some(at) = ActivityType::get_with_id(at.id, &self.conn)? else {
            return Err(Error::ActivityTypeNotFound(at.id.to_string()));
        };

        let activity = Activity::new(project.id, at.id, description);
        activity.create(&self.conn)?;

        Ok(activity)
    }

    pub fn cancel_running_actvity(&self, project: &Project) -> Result<(), Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let Some(activity) = self.stop_running_activity(&project)? else {
            return Ok(());
        };

        activity.delete(&self.conn)?;

        Ok(())
    }

    pub fn stop_running_activity(&self, project: &Project) -> Result<Option<Activity>, Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let mut activities: Vec<_> = Activity::get_if_running(&self.conn)?
            .into_iter()
            .filter(|a| a.project == project.id)
            .collect();

        if activities.len() == 1 {
            let mut activity = activities.remove(0);
            activity.end = Some(Utc::now());
            activity.update(&self.conn)?;
            return Ok(Some(activity));
        }

        Ok(None)
    }

    pub fn get_running_activity(&self, project: &Project) -> Result<Option<Activity>, Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let mut activities: Vec<_> = Activity::get_if_running(&self.conn)?
            .into_iter()
            .filter(|a| a.project == project.id)
            .collect();

        if activities.len() == 1 {
            return Ok(Some(activities.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_activity_with_id(&self, id: u64) -> Result<Option<Activity>, Error> {
        let activity = Activity::get_with_id(id, &self.conn)?;
        Ok(activity)
    }

    pub fn delete_activity(&self, activity: Activity) -> Result<(), Error> {
        let Some(activity) = Activity::get_with_id(activity.id, &self.conn)? else {
            return Err(Error::ActivityNotFound(activity.id.to_string()));
        };

        activity.delete(&self.conn)?;

        Ok(())
    }

    pub fn update_activity(&self, activity: &Activity) -> Result<(), Error> {
        let Some(activity) = Activity::get_with_id(activity.id, &self.conn)? else {
            return Err(Error::ActivityNotFound(activity.id.to_string()));
        };

        activity.update(&self.conn)?;

        Ok(())
    }

    pub fn get_activities(&self, project: &Project) -> Result<Vec<Activity>, Error> {
        let Some(_project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let activities = Activity::get_all(&self.conn)?
            .into_iter()
            .filter(|a| a.project == project.id)
            .collect();

        Ok(activities)
    }
}

// Repos
impl DataStore {
    pub fn create_repo(&self, project: &Project, path: &Path) -> Result<Repo, Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let repo = Repo::new(PathBuf::from(path), project.id);
        repo.create(&self.conn)?;

        Ok(repo)
    }

    pub fn get_repo(&self, path: &Path) -> Result<Option<Repo>, Error> {
        let mut repos = Repo::get_with_path(path, &self.conn)?;

        if repos.len() == 1 {
            return Ok(Some(repos.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn delete_repo(&self, repo: Repo) -> Result<(), Error> {
        let Some(repo) = Repo::get_with_id(repo.id, &self.conn)? else {
            return Err(Error::RepoNotFound(repo.id.to_string()));
        };

        repo.delete(&self.conn)?;

        Ok(())
    }

    pub fn update_repo(&self, repo: &Repo) -> Result<(), Error> {
        let Some(repo) = Repo::get_with_id(repo.id, &self.conn)? else {
            return Err(Error::RepoNotFound(repo.id.to_string()));
        };

        repo.update(&self.conn)?;

        Ok(())
    }

    pub fn get_repos(&self, project: &Project) -> Result<Vec<Repo>, Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let repos = Repo::get_all(&self.conn)?
            .into_iter()
            .filter(|r| r.project == project.id)
            .collect();

        Ok(repos)
    }
}
