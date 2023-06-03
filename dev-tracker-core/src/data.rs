use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::model::activitytype::ActivityType;
use crate::model::project::Project;
use crate::model::{activity, activitytype, project};
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
}

// Project
impl DataStore {
    pub fn add_project(&self, project: &Project) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO projects (name, path) VALUES (?1, ?2)",
            (&project.name, &project.path.display().to_string()),
        )?;

        Ok(())
    }

    pub fn delete_project(&self, project: Project) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM projects WHERE id=?1",
            &[&project.id.to_string()],
        )?;

        Ok(())
    }

    pub fn update_project(&self, project: &Project) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE projects SET name=?2, path=?3 WHERE id=?1",
            (
                &project.id,
                &project.name,
                &project.path.display().to_string(),
            ),
        )?;

        Ok(())
    }

    pub fn get_project_with_id(&self, id: u64) -> Result<Option<Project>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, path FROM projects WHERE id=?1")?;
        let mut projects: Vec<Project> = stmt
            .query_map([&id.to_string()], |row| {
                let path: String = row.get(2)?;
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: PathBuf::from(path),
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if projects.len() == 1 {
            return Ok(Some(projects.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_project_with_name(&self, name: &str) -> Result<Option<Project>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, path FROM projects WHERE name=?1")?;
        let mut projects: Vec<Project> = stmt
            .query_map([name], |row| {
                let path: String = row.get(2)?;
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: PathBuf::from(path),
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if projects.len() == 1 {
            return Ok(Some(projects.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, name, path FROM projects")?;
        let projects: Vec<_> = stmt
            .query_map([], |row| {
                let path: String = row.get(2)?;
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: PathBuf::from(path),
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(projects)
    }
}

// ActivityType
impl DataStore {
    pub fn add_activitytype(&self, at: ActivityType) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO activitytypes (name, description) VALUES (?1, ?2)",
            (&at.name, &at.description),
        )?;

        Ok(())
    }

    // TODO: should this check for activities using this type before
    // deleting, or should that be done by the application?
    pub fn delete_activitytype(&self, at: ActivityType) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM activitytypes WHERE id=?1",
            &[&at.id.to_string()],
        )?;

        Ok(())
    }

    pub fn update_activitytype(&self, at: &ActivityType) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE activitytypes SET name=?2, description=?3 WHERE id=?1",
            (&at.id, &at.name, &at.description),
        )?;

        Ok(())
    }

    pub fn get_activitytype_with_id(&self, id: u64) -> Result<Option<ActivityType>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM activitytypes WHERE id=?1")?;
        let mut ats: Vec<ActivityType> = stmt
            .query_map([&id.to_string()], |row| {
                Ok(ActivityType {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if ats.len() == 1 {
            return Ok(Some(ats.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_activitytype_with_name(&self, name: &str) -> Result<Option<ActivityType>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM activitytypes WHERE name=?1")?;
        let mut ats: Vec<ActivityType> = stmt
            .query_map([name], |row| {
                Ok(ActivityType {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if ats.len() == 1 {
            return Ok(Some(ats.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub fn get_activitytypes(&self) -> Result<Vec<ActivityType>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM activitytypes")?;
        let ats: Vec<_> = stmt
            .query_map([], |row| {
                Ok(ActivityType {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(ats)
    }
}

impl DataStore {
    fn init_tables(&self) -> Result<(), Error> {
        project::init_table(&self.conn)?;
        activity::init_table(&self.conn)?;
        activitytype::init_table(&self.conn)?;

        Ok(())
    }
}
