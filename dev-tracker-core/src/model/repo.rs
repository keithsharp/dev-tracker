use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use rusqlite::Connection;

use crate::model::Project;

#[derive(Debug)]
pub struct Repo {
    pub(crate) id: u64,
    pub(crate) project: u64,
    pub(crate) path: PathBuf,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl Repo {
    pub fn new(path: PathBuf, project: &Project) -> Self {
        Self {
            id: 0,
            project: project.id,
            path,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn project(&self) -> u64 {
        self.project
    }

    pub fn set_project(&mut self, project: &Project) {
        self.project = project.id;
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }
}

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
