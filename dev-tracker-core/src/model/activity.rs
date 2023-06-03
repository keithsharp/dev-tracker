use std::fmt::Display;

use chrono::{DateTime, Duration, Local, Utc};
use rusqlite::Connection;

use crate::model::ActivityType;
use crate::model::Project;

#[derive(Debug)]
pub struct Activity {
    pub(crate) id: u64,
    pub(crate) project: u64,
    pub(crate) atype: u64,
    pub(crate) description: String,
    pub(crate) start: DateTime<Utc>,
    pub(crate) end: Option<DateTime<Utc>>,
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let local_start: DateTime<Local> = DateTime::from(self.start);
        if let Some(end) = self.end {
            let local_end: DateTime<Local> = DateTime::from(end);
            let dur = self
                .duration()
                .expect("we have an end so we should have a duration");
            let hours = dur.num_hours();
            let minutes = dur.num_minutes() % 60;

            let hours = match hours {
                0 => String::new(),
                1 => "1 hour".to_string(),
                h => format!("{} hours", h),
            };

            let minutes = match minutes {
                0 => String::new(),
                1 => "1 minute".to_string(),
                m => format!("{} minutes", m),
            };

            let duration = match (hours.is_empty(), minutes.is_empty()) {
                (false, false) => format!("{} {}", hours, minutes),
                (true, false) => minutes,
                (false, true) => hours,
                (true, true) => "less than a minutes".to_string(),
            };

            write!(
                f,
                "from {} until {}, total time {}",
                local_start.format("%I:%M%P on %A %d %B %Y").to_string(),
                local_end.format("%I:%M%P on %A %d %B %Y").to_string(),
                duration
            )?;
        } else {
            write!(
                f,
                "started at {}, and is still running",
                local_start.format("%I:%M%P on %A %d %B %Y").to_string()
            )?;
        }
        Ok(())
    }
}

impl Activity {
    pub fn new(project: &Project, atype: ActivityType) -> Self {
        Self {
            id: 0,
            project: project.id(),
            atype: atype.id,
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

    pub fn atype(&self) -> u64 {
        self.atype
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end
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
        "CREATE TABLE IF NOT EXISTS activities (
            id          INTEGER PRIMARY KEY,
            project     INTEGER NOT NULL,
            atype       u64,
            description TEXT,
            start       DATETIME NOT NULL,
            end         DATETIME
        )",
        (),
    )?;
    Ok(())
}
