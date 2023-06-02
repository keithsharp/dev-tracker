use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;

use crate::model::Project;

#[derive(Debug)]
pub enum ActivityType {
    Coding,
    Research,
}

impl Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let activity = match self {
            ActivityType::Coding => "coding",
            ActivityType::Research => "researching",
        };
        write!(f, "{}", activity)
    }
}

impl FromStr for ActivityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "coding" => Ok(ActivityType::Coding),
            "researching" => Ok(ActivityType::Research),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ActivityTypeParseError(String);

impl Error for ActivityTypeParseError {}

impl Display for ActivityTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityTypeParseError(s) => write!(f, "could not parse '{}' into an ActivityType", s),
        }
    }
}

#[derive(Debug)]
pub struct Activity {
    pub(crate) id: u64,
    pub(crate) project: u64,
    pub(crate) atype: ActivityType,
    pub(crate) description: String,
    pub(crate) start: DateTime<Utc>,
    pub(crate) end: Option<DateTime<Utc>>,
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = match self.end {
            Some(end) => end.to_rfc3339(),
            None => "None".to_string(),
        };
        write!(
            f,
            "{} from {} until {}",
            self.atype,
            self.start.to_rfc3339(),
            end
        )
    }
}

impl Activity {
    pub fn new(project: &Project, atype: ActivityType) -> Self {
        Self {
            id: 0,
            project: project.id(),
            atype,
            description: String::default(),
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

    pub fn atype(&self) -> &ActivityType {
        &self.atype
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start
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

pub(crate) fn init_table(conn: &Connection) -> Result<(), crate::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activitys (
            id          INTEGER PRIMARY KEY,
            project     INTEGER NOT NULL,
            atype       TEXT NOT NULL,
            description TEXT,
            start       DATETIME NOT NULL,
            end         DATETIME
        )",
        (),
    )?;
    Ok(())
}
