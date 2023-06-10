use std::fmt::Display;

use rusqlite::Connection;

use crate::Error;

pub(crate) fn init_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.name)
    }
}

impl Project {
    pub fn new(name: String) -> Self {
        Self { id: 0, name }
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
}

impl Project {
    pub(crate) fn create(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute("INSERT INTO projects (name) VALUES (?1)", [&self.name])?;
        Ok(())
    }

    pub(crate) fn get_with_id(id: u64, conn: &Connection) -> Result<Option<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, name FROM projects WHERE id=?1")?;
        let mut projects: Vec<Project> = stmt
            .query_map([&id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if projects.len() == 1 {
            Ok(Some(projects.remove(0)))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn get_with_name(name: &str, conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, name FROM projects WHERE name=?1")?;
        let projects: Vec<Project> = stmt
            .query_map([&name], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(projects)
    }

    pub(crate) fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, name FROM projects")?;
        let projects: Vec<Project> = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(projects)
    }

    pub(crate) fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE projects SET name=?2 WHERE id=?1",
            (&self.id, &self.name),
        )?;

        Ok(())
    }

    pub(crate) fn delete(self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM projects WHERE id=?1", [&self.id])?;
        Ok(())
    }
}
