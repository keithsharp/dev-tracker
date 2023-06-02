use std::fmt::Display;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::Error;

#[derive(Debug)]
pub struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) path: PathBuf,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { id: 0, name, path }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }
}

pub(crate) fn init_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            path        TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}
