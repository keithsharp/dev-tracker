use std::fmt::Display;

use rusqlite::Connection;

#[derive(Debug)]
pub struct ActivityType {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

impl Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

pub(crate) fn init_table(conn: &Connection) -> Result<(), crate::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activitytypes (
            id          INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            description TEXT
        )",
        (),
    )?;
    Ok(())
}
