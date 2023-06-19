use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokei::Languages;

use crate::model::Project;

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,

    project_name: String,
    project_description: Option<String>,

    pub(crate) activities: Vec<Activity>,
    pub(crate) counts: HashMap<String, Vec<Count>>,
}

impl Report {
    pub fn new(
        project: &Project,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            start,
            end,
            project_name: project.name.clone(),
            project_description: None,
            activities: Vec::new(),
            counts: HashMap::new(),
        }
    }

    pub fn print(&self) {
        let start_phrase = match self.start {
            Some(start) => start.format("%A %d %B %Y").to_string(),
            None => match self.activities.get(0) {
                Some(activity) => activity.start.format("%A %d %B %Y").to_string(),
                None => "beginning".to_string(),
            },
        };
        let end_phrase = match self.end {
            Some(end) => end.format("%A %d %B %Y").to_string(),
            None => Utc::now().format("%A %d %B %Y").to_string(),
        };
        println!(
            "Report for {} covering period from {} to {}.",
            self.project_name, start_phrase, end_phrase
        );

        if self.activities.is_empty() {
            println!("\n  There were no activities recorded.");
        } else {
            let total_time: i64 = self.activities.iter().map(|a| a.minutes).sum();
            println!(
                "\n  There were {} activities recorded with a total time of {}.",
                self.activities.len(),
                minutes_to_str(total_time)
            );

            for activity in &self.activities {
                println!(
                    "    {} for {} on {}.",
                    activity.name,
                    minutes_to_str(activity.minutes),
                    activity.start.format("%A %d %B %Y")
                );
            }
        }

        if !self.counts.is_empty() {
            let loc: usize = self
                .counts
                .values()
                .map(|vc| vc.last().map_or(0, |c| c.count.total().code))
                .sum();
            println!(
                "\n  The total lines of code in the repositories is {}.",
                loc
            );
        }
        for (path, counts) in &self.counts {
            if let Some(count) = counts.last() {
                println!(
                    "    {} has {} lines of code",
                    path,
                    count.count.total().code
                );
            } else {
                println!("    {} has no count of lines of code.", path);
            }
        }

        println!();
    }
}

fn minutes_to_str(minutes: i64) -> String {
    let hours = minutes / 60;
    let minutes = minutes % 60;

    let hours = if hours == 1 {
        format!("1 hour ")
    } else if hours > 1 {
        format!("{} hours ", hours)
    } else {
        "zero hours ".to_string()
    };

    let minutes = if minutes == 1 {
        format!("1 minute")
    } else if minutes > 1 {
        format!("{} minutes", minutes)
    } else {
        "zero minutes".to_string()
    };

    format!("{}{}", hours, minutes)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    pub(crate) name: String,
    pub(crate) start: DateTime<Utc>,
    pub(crate) minutes: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Count {
    pub(crate) path: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) count: Languages,
}
