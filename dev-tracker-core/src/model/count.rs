use std::fmt::Display;

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::Error;

pub(crate) fn init_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS counts (
            id      INTEGER PRIMARY KEY,
            repo    INTEGER NOT NULL,
            date    DATETIME NOT NULL,
            count   INTEGER NOT NULL
        )",
        (),
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Count {
    pub(crate) id: u64,
    pub(crate) repo: u64,
    pub(crate) date: DateTime<Utc>,
    pub(crate) count: u64,
}

impl Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.id,
            self.repo,
            self.date.format("%I:%M%P %A %d %B %Y"),
            self.count
        )
    }
}

impl Count {
    pub fn new(repo: u64, date: DateTime<Utc>, count: u64) -> Self {
        Self {
            id: 0,
            repo,
            date,
            count,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn repo(&self) -> u64 {
        self.repo
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

impl Count {
    pub(crate) fn create(&self, conn: &Connection) -> Result<(), Error> {
        conn.execute(
            "INSERT INTO counts (repo, date, count) VALUES (?1, ?2, ?3)",
            (&self.repo, &self.date, &self.count),
        )?;
        Ok(())
    }

    pub(crate) fn get_with_id(id: u64, conn: &Connection) -> Result<Option<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, repo, date, count FROM counts WHERE id=?1")?;
        let mut counts: Vec<Count> = stmt
            .query_map([&id], |row| {
                Ok(Count {
                    id: row.get(0)?,
                    repo: row.get(1)?,
                    date: row.get(2)?,
                    count: row.get(3)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        if counts.len() == 1 {
            return Ok(Some(counts.remove(0)));
        } else {
            return Ok(None);
        }
    }

    pub(crate) fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut stmt = conn.prepare("SELECT id, repo, date, count FROM counts")?;
        let counts: Vec<Count> = stmt
            .query_map([], |row| {
                Ok(Count {
                    id: row.get(0)?,
                    repo: row.get(1)?,
                    date: row.get(2)?,
                    count: row.get(3)?,
                })
            })?
            .filter_map(|p| p.ok())
            .collect();

        Ok(counts)
    }

    pub(crate) fn delete(self, conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM counts WHERE id=?1", &[&self.id])?;
        Ok(())
    }
}
