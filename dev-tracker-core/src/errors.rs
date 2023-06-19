use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    SerdeJson(serde_json::Error),
    ProjectNotFound(String),
    ProjectAlreadyExists(String),
    ActivityTypeNotFound(String),
    ActivityTypeAlreadyExists(String),
    ActivityTypeInUse(String),
    RepoNotFound(String),
    RepoAlreadyExists(String),
    ActivityNotFound(String),
    ActivityAlreadyExists(String),
    RunningActivityAlreadyExists(String),
    CountNotFound(String),
    ReportError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (module, e) = match self {
            Error::Rusqlite(e) => ("rusqlite", e.to_string()),
            Error::SerdeJson(e) => ("serde", e.to_string()),
            Error::ProjectNotFound(item) => ("notfound", format!("project '{}' not found", item)),
            Error::ProjectAlreadyExists(item) => (
                "alreadyexists",
                format!("project '{}' already exists", item),
            ),
            Error::ActivityTypeNotFound(item) => {
                ("notfound", format!("activity type '{}' not found", item))
            }
            Error::ActivityTypeAlreadyExists(item) => (
                "alreadyexists",
                format!("activity type '{}' already exists", item),
            ),
            Error::ActivityTypeInUse(item) => {
                ("inuse", format!("activity type '{}' is in use", item))
            }
            Error::RepoNotFound(item) => ("notfound", format!("repo '{}' not found", item)),
            Error::RepoAlreadyExists(item) => {
                ("alreadyexists", format!("repo '{}' already exists", item))
            }
            Error::ActivityNotFound(item) => ("notfound", format!("activity '{}' not found", item)),
            Error::ActivityAlreadyExists(item) => (
                "alreadyexists",
                format!("activity '{}' already exists", item),
            ),
            Error::RunningActivityAlreadyExists(item) => (
                "alreadyexists",
                format!("runningactivity already exists in project '{}'", item),
            ),
            Error::CountNotFound(item) => ("notfound", format!("count '{}' not found", item)),
            Error::ReportError(item) => ("report", format!("could not create report: '{}'", item)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Rusqlite(e) => Some(e),
            Error::SerdeJson(e) => Some(e),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::Rusqlite(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}
