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

    if ActivityType::get_with_id(0, conn)?.is_none() {
        conn.execute(
        "INSERT INTO activitytypes (id, name, description) VALUES (0, 'Unknown', 'Unknown activity type')",
    ())?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct ActivityType {
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
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: 0,
            name,
            description,
        }
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

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, description: Option<String>) {
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

    pub(crate) fn get_with_id(id: u64, conn: &Connection) -> Result<Option<Self>, Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, description FROM activitytypes WHERE id=?1")?;
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
            Ok(Some(ats.remove(0)))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn get_with_name(name: &str, conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, description FROM activitytypes WHERE name=?1")?;
        let ats: Vec<ActivityType> = stmt
            .query_map([&name], |row| {
                Ok(ActivityType {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(1)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(ats)
    }

    pub(crate) fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, name, description FROM activitytypes")?;
        let ats: Vec<ActivityType> = stmt
            .query_map([], |row| {
                Ok(ActivityType {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(1)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(ats)
    }

    pub(crate) fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE activitytypes SET name=?2 WHERE id=?1",
            (&self.id, &self.name),
        )?;

        Ok(())
    }

    pub(crate) fn delete(self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM activitytypes WHERE id=?1", [&self.id])?;
        Ok(())
    }
}
