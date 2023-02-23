use std::error::Error;

pub trait CustomError: Error + Send + Sync + 'static {}

#[derive(Debug)]
pub enum FileError {
    FileOpenError(std::io::Error),
    PathNotGiven
}

#[derive(Debug)]
pub enum TermError {
    TermNotGiven
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::FileOpenError(e) => write!(f, "Fail at open file: {e}"),
            FileError::PathNotGiven => write!(f, "Path for the file was not given.")
        }
    }
}

impl std::fmt::Display for TermError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermError::TermNotGiven => write!(f, "Search term was not given.")
        }
    }
}

impl Error for FileError {}
impl CustomError for FileError {}

impl Error for TermError {}
impl CustomError for TermError {}
