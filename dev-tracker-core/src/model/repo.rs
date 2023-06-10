use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use rusqlite::Connection;

use crate::Error;

pub(crate) fn init_table(conn: &Connection) -> Result<(), crate::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS repos (
            id          INTEGER PRIMARY KEY,
            project     INTEGER NOT NULL,
            path        TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Repo {
    pub(crate) id: u64,
    pub(crate) project: u64,
    pub(crate) path: PathBuf,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.id, self.project, self.path.display())
    }
}

impl Repo {
    pub fn new(path: PathBuf, project: u64) -> Self {
        Self {
            id: 0,
            project,
            path,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn project(&self) -> u64 {
        self.project
    }

    pub fn set_project(&mut self, project: u64) {
        self.project = project;
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }
}

impl Repo {
    pub(crate) fn create(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT INTO repos (project, path) VALUES (?1, ?2)",
            (&self.project, &self.path.display().to_string()),
        )?;
        Ok(())
    }

    pub(crate) fn get_with_id(id: u64, conn: &Connection) -> Result<Option<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, project, path FROM repos WHERE id=?1")?;
        let mut repos: Vec<Repo> = stmt
            .query_map([&id], |row| {
                let path: String = row.get(2)?;
                Ok(Repo {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    path: PathBuf::from(path),
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if repos.len() == 1 {
            Ok(Some(repos.remove(0)))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn get_with_path(path: &Path, conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, project, path FROM repos WHERE path=?1")?;
        let repos: Vec<Repo> = stmt
            .query_map([&path.display().to_string()], |row| {
                let path: String = row.get(2)?;
                Ok(Repo {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    path: PathBuf::from(path),
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(repos)
    }

    pub(crate) fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, project, path FROM repos")?;
        let repos: Vec<Repo> = stmt
            .query_map([], |row| {
                let path: String = row.get(2)?;
                Ok(Repo {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    path: PathBuf::from(path),
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(repos)
    }

    pub(crate) fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE repos SET project=?2, path=?3 WHERE id=?1",
            (&self.id, &self.project, &self.path.display().to_string()),
        )?;

        Ok(())
    }

    pub(crate) fn delete(self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM repos WHERE id=?1", [&self.id])?;
        Ok(())
    }
}
