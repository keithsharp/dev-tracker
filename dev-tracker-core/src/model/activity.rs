use std::fmt::Display;

use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;

use crate::Error;

pub(crate) fn init_table(conn: &Connection) -> Result<(), crate::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activities (
            id          INTEGER PRIMARY KEY,
            project     INTEGER NOT NULL,
            atype       INTEGER NOT NULL,
            description TEXT,
            start       DATETIME NOT NULL,
            end         DATETIME
        )",
        (),
    )?;
    Ok(())
}
#[derive(Clone, Debug)]
pub struct Activity {
    pub(crate) id: u64,
    pub(crate) project: u64,
    pub(crate) atype: u64,
    pub(crate) description: Option<String>,
    pub(crate) start: DateTime<Utc>,
    pub(crate) end: Option<DateTime<Utc>>,
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(end) = self.end {
            write!(
                f,
                "{} {} {} {} {}",
                self.id,
                self.project,
                self.atype,
                self.start.format("%I:%M%P %A %d %B %Y"),
                end.format("")
            )
        } else {
            write!(
                f,
                "{} {} {} {} running",
                self.id,
                self.project,
                self.atype,
                self.start.format("%I:%M%P %A %d %B %Y")
            )
        }
    }
}

impl Activity {
    pub fn new(project: u64, atype: u64, description: Option<String>) -> Self {
        Self {
            id: 0,
            project: project,
            atype: atype,
            description,
            start: Utc::now(),
            end: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn project(&self) -> u64 {
        self.project
    }

    pub fn set_project(&mut self, id: u64) {
        self.project = id;
    }

    pub fn atype(&self) -> u64 {
        self.atype
    }

    pub fn set_atype(&mut self, id: u64) {
        self.atype = id;
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end
    }

    pub fn set_end_time(&mut self, end: Option<DateTime<Utc>>) {
        self.end = end;
    }

    // TODO: should this throw an error rather than silently overwrite
    pub fn stop(&mut self) {
        self.end = Some(Utc::now());
    }

    pub fn duration(&self) -> Option<Duration> {
        let Some(end) = self.end else {
            return None;
        };

        Some(end - self.start)
    }
}

impl Activity {
    pub(crate) fn create(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT INTO activities (project, atype, description, start) VALUES (?1, ?2, ?3, ?4)",
            (&self.project, &self.atype, &self.description, &self.start),
        )?;
        Ok(())
    }

    pub(crate) fn get_with_id(id: u64, conn: &Connection) -> Result<Option<Self>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, project, atype, description, start, end FROM activities WHERE id=?1",
        )?;

        let mut activities: Vec<_> = stmt
            .query_map([id], |row| {
                Ok(Activity {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    atype: row.get(2)?,
                    description: row.get(3)?,
                    start: row.get(4)?,
                    end: row.get(5)?,
                })
            })?
            .filter_map(|a| a.ok())
            .collect();

        if activities.len() == 1 {
            return Ok(Some(activities.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub(crate) fn get_if_running(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare(
            "SELECT id, project, atype, description, start FROM activities WHERE end IS NULL",
        )?;

        let activities: Vec<_> = stmt
            .query_map([], |row| {
                Ok(Activity {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    atype: row.get(2)?,
                    description: row.get(3)?,
                    start: row.get(4)?,
                    end: row.get(5)?,
                })
            })?
            .filter_map(|a| a.ok())
            .collect();

        Ok(activities)
    }

    pub(crate) fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt =
            conn.prepare("SELECT id, project, atype, description, start, end FROM activities")?;

        let activities: Vec<_> = stmt
            .query_map([], |row| {
                Ok(Activity {
                    id: row.get(0)?,
                    project: row.get(1)?,
                    atype: row.get(2)?,
                    description: row.get(3)?,
                    start: row.get(4)?,
                    end: row.get(5)?,
                })
            })?
            .filter_map(|a| a.ok())
            .collect();

        Ok(activities)
    }

    pub(crate) fn update(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "UPDATE activities SET project=?2, atype=?3, description=?4, start=?5, end=?6 WHERE id=?1",
            (&self.id, &self.project, &self.atype, &self.description, &self.start, &self.end),
        )?;

        Ok(())
    }

    pub(crate) fn delete(self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "DELETE FROM activities WHERE id=?1",
            &[&self.id.to_string()],
        )?;
        Ok(())
    }
}
