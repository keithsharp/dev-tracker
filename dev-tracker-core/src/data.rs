use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use tokei::{Config, Languages};

use crate::model::activity::Activity;
use crate::model::activitytype::ActivityType;
use crate::model::count::Count;
use crate::model::project::Project;
use crate::model::repo::Repo;
use crate::model::{activity, activitytype, count, project, repo};
use crate::report::{self, Report};
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
        count::init_table(&self.conn)?;

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
            self.delete_activity(activity)?;
        }

        let repos: Vec<_> = Repo::get_all(&self.conn)?
            .into_iter()
            .filter(|r| r.project == project.id)
            .collect();

        for repo in repos {
            self.delete_repo(repo)?;
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
            Ok(Some(projects.remove(0)))
        } else {
            Ok(None)
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

        let activities: Vec<Activity> = Activity::get_all(&self.conn)?
            .into_iter()
            .filter(|a| a.atype == at.id)
            .collect();

        for mut activity in activities {
            activity.set_atype(0);
            activity.update(&self.conn)?;
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
            Ok(Some(ats.remove(0)))
        } else {
            Ok(None)
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
            Ok(Some(activities.remove(0)))
        } else {
            Ok(None)
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
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
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
            Ok(Some(repos.remove(0)))
        } else {
            Ok(None)
        }
    }

    pub fn get_repo_with_id(&self, id: u64) -> Result<Option<Repo>, Error> {
        let repo = Repo::get_with_id(id, &self.conn)?;
        Ok(repo)
    }

    pub fn delete_repo(&self, repo: Repo) -> Result<(), Error> {
        let Some(repo) = Repo::get_with_id(repo.id, &self.conn)? else {
            return Err(Error::RepoNotFound(repo.id.to_string()));
        };

        let counts: Vec<_> = Count::get_all(&self.conn)?
            .into_iter()
            .filter(|c| c.repo == repo.id)
            .collect();

        for count in counts {
            self.delete_count(count)?;
        }

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

// Count
impl DataStore {
    pub fn create_count(&self, repo: &Repo) -> Result<Count, Error> {
        let Some(repo) = Repo::get_with_id(repo.id, &self.conn)? else {
            return Err(Error::RepoNotFound(repo.id.to_string()));
        };
        let paths = vec![repo.path.display().to_string()];

        let date = Utc::now();

        let excluded = &["target"];
        let config = Config::default();
        let mut languages = Languages::new();
        languages.get_statistics(&paths[..], excluded, &config);

        let count = Count::new(repo.id, date, languages);
        count.create(&self.conn)?;

        Ok(count)
    }

    pub fn get_count_with_id(&self, id: u64) -> Result<Option<Count>, Error> {
        let count = Count::get_with_id(id, &self.conn)?;
        Ok(count)
    }

    pub fn get_latest_count(&self, repo: &Repo) -> Result<Option<Count>, Error> {
        let mut counts = self.get_counts(repo)?;
        #[allow(clippy::comparison_chain)]
        counts.sort_by(|a, b| {
            if a.date < b.date {
                Ordering::Less
            } else if a.date == b.date {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

        if !counts.is_empty() {
            Ok(Some(counts.remove(counts.len() - 1)))
        } else {
            Ok(None)
        }
    }

    pub fn delete_count(&self, count: Count) -> Result<(), Error> {
        let Some(count) = Count::get_with_id(count.id, &self.conn)? else {
            return Err(Error::CountNotFound(count.id.to_string()));
        };

        count.delete(&self.conn)?;

        Ok(())
    }

    pub fn get_counts(&self, repo: &Repo) -> Result<Vec<Count>, Error> {
        let Some(repo) = Repo::get_with_id(repo.id, &self.conn)? else {
            return Err(Error::RepoNotFound(repo.id.to_string()));
        };

        let counts = Count::get_all(&self.conn)?
            .into_iter()
            .filter(|c| c.repo == repo.id)
            .collect();

        Ok(counts)
    }

    pub fn get_total_loc(&self, count: &Count) -> Result<u64, Error> {
        let Some(count) = Count::get_with_id(count.id, &self.conn)? else {
            return Err(Error::CountNotFound(count.id.to_string()));
        };

        Ok(count.count.total().code as u64)
    }
}

// Report
impl DataStore {
    pub fn create_report(
        &self,
        project: &Project,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<Report, Error> {
        let Some(project) = Project::get_with_id(project.id, &self.conn)? else {
            return Err(Error::ProjectNotFound(project.id.to_string()));
        };

        let mut report = Report::new(&project, start, end);

        let activities = self.get_report_activities(&project, start, end)?;
        report.activities = activities;

        let counts = self.get_report_counts(&project, start, end)?;
        report.counts = counts;

        Ok(report)
    }

    fn get_report_activities(
        &self,
        project: &Project,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<Vec<report::Activity>, Error> {
        let activities = self.get_activities(&project)?;

        let activities = match (start, end) {
            (None, None) => activities,
            (None, Some(end)) => activities.into_iter().filter(|a| a.start <= end).collect(),
            (Some(start), None) => activities
                .into_iter()
                .filter(|a| a.start >= start)
                .collect(),
            (Some(start), Some(end)) => activities
                .into_iter()
                .filter(|a| a.start >= start && a.start <= end)
                .collect(),
        };

        let activities: Vec<_> = activities
            .into_iter()
            .map(|a| {
                let at = ActivityType::get_with_id(a.atype, &self.conn)
                    .expect("should always be able to access the datastore");
                let name = at.map_or_else(|| "Unknown".to_string(), |at| at.name);
                let start = a.start;
                let end = a.end.unwrap_or(Utc::now());
                let duration = end - start;
                report::Activity {
                    name,
                    start,
                    minutes: duration.num_minutes(),
                }
            })
            .collect();

        Ok(activities)
    }

    fn get_report_counts(
        &self,
        project: &Project,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<HashMap<String, Vec<report::Count>>, Error> {
        let mut all_counts = HashMap::new();

        let repos = self.get_repos(&project)?;
        for repo in repos {
            let counts = self.get_counts(&repo)?;

            let counts = match (start, end) {
                (None, None) => counts,
                (None, Some(end)) => counts.into_iter().filter(|c| c.date <= end).collect(),
                (Some(start), None) => counts.into_iter().filter(|c| c.date >= start).collect(),
                (Some(start), Some(end)) => counts
                    .into_iter()
                    .filter(|c| c.date >= start && c.date <= end)
                    .collect(),
            };

            let path = repo.path.display().to_string();

            let counts: Vec<_> = counts
                .into_iter()
                .map(|c| report::Count {
                    path: path.clone(),
                    date: c.date,
                    count: c.count,
                })
                .collect();

            all_counts.insert(path, counts);
        }

        Ok(all_counts)
    }
}
