use std::fmt::Display;

use rusqlite::Connection;

use crate::Error;

pub(crate) fn init_table(conn: &Connection) -> Result<(), crate::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activitytypes (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            description TEXT
        )",
        (),
    )?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct ActivityType {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

impl Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.name)
    }
}

impl ActivityType {
    pub(crate) fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: 0,
            name,
            description,
        }
    }

    pub(crate) fn id(&self) -> u64 {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub(crate) fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}

impl ActivityType {
    pub(crate) fn create(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT INTO activitytypes (name, description) VALUES (?1, ?2)",
            (&self.name, &self.description),
        )?;
        Ok(())
    }

    pub(crate) fn read(id: u64, conn: &Connection) -> Result<Self, Error> {
        let mut stmt = conn.prepare("SELECT id, name, description FROM projects WHERE id=?1")?;
        let mut ats: Vec<ActivityType> = stmt
            .query_map([&id], |row| {
                Ok(ActivityType {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(1)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if ats.len() == 1 {
            return Ok(ats.remove(0));
        } else {
            return Err(Error::ActivityTypeNotFound(id));
        }
    }

    pub(crate) fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE activitytypes SET name=?2 WHERE id=?1",
            (&self.id, &self.name),
        )?;

        Ok(())
    }

    pub(crate) fn delete(self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM activitytypes WHERE id=?1", &[&self.id])?;
        Ok(())
    }
}
