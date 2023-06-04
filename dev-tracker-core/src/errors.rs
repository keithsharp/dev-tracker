use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    ProjectNotFound(u64),
    ActivityTypeNotFound(u64),
    RepoNotFound(u64),
    ActivityNotFound(u64),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (module, e) = match self {
            Error::Rusqlite(e) => ("rusqlite", e.to_string()),
            Error::ProjectNotFound(id) => ("notfound", format!("project ID {}", id)),
            Error::ActivityTypeNotFound(id) => ("notfound", format!("activity type ID {}", id)),
            Error::RepoNotFound(id) => ("notfound", format!("repo ID {}", id)),
            Error::ActivityNotFound(id) => ("notfound", format!("activity ID {}", id)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Rusqlite(e) => Some(e),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::Rusqlite(e)
    }
}
