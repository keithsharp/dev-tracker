use std::fmt::Display;

#[derive(Debug)]
pub enum ArgParserError {
    ChronoParserError(chrono::format::ParseError),
    // DateParserError(String),
    // TimeParserError(String),
}

impl Display for ArgParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (module, e) = match self {
            ArgParserError::ChronoParserError(e) => ("chrono", e.to_string()),
            // ArgParserError::DateParserError(token) => {
            //     ("datetime", format!("failed to parse date from '{}'", token))
            // }
            // ArgParserError::TimeParserError(token) => {
            //     ("datetime", format!("failed to parse time from '{}'", token))
            // }
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl std::error::Error for ArgParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ArgParserError::ChronoParserError(e) => Some(e),
            // _ => None,
        }
    }
}

impl From<chrono::format::ParseError> for ArgParserError {
    fn from(e: chrono::format::ParseError) -> Self {
        ArgParserError::ChronoParserError(e)
    }
}
