use std::fmt::{self,Display};
use serde_json;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum CreationError {
    EmptyString,
    DateError,
    FolderErr,
    FileCreation,
} 

impl Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            CreationError::EmptyString => write!(f,"Empty string"),
            CreationError::DateError => write!(f,"Date Error"),
            CreationError::FolderErr => write!(f,"Directory error"),
            CreationError::FileCreation => write!(f,"File creation"),
        }
    } 
}

#[derive(Debug)]
pub enum TodoFileError {
    OpenFile(std::io::Error),
    ClearingError,
    CopyError,
    WriteError(serde_json::Error),
    Modify(String)
}