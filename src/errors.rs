use std::num::ParseIntError;

#[derive(Debug)]
pub enum ArgParseError {
    NoTitle,
    NoIndex,
    NoDescription,
    NoListName,
    BadIndex(ParseIntError),
}

impl From<ParseIntError> for ArgParseError {
    fn from(v: ParseIntError) -> Self {
        Self::BadIndex(v)
    }
}

impl std::error::Error for ArgParseError {}
impl std::fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArgParseError::BadIndex(e) => e.to_string(),
                ArgParseError::NoTitle => "No title provided".to_string(),
                ArgParseError::NoIndex => "No index provided".to_string(),
                ArgParseError::NoDescription => "No description provided".to_string(),
                ArgParseError::NoListName => "No list name provided".to_string(),
            }
        )
    }
}
