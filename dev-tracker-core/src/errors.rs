use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (module, e) = match self {
            Error::Rusqlite(e) => ("rusqlite", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Error::Rusqlite(e) => e,
        })
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::Rusqlite(e)
    }
}
