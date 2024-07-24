// * Error types 

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
    Modify(String),
    NotAPriority(String),
}

impl Display for TodoFileError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            // TodoFileError::OpenFile(e) => write!(f, "Error while opening the save file : {} ", e),
            TodoFileError::OpenFile(_) => write!(f, "Error while opening the save file "),
            TodoFileError::ClearingError => write!(f,"Error while clearing the file."),
            TodoFileError::CopyError => write!(f,"Error while copying the file"),
            TodoFileError::WriteError(_) => write!(f,"Error while parsing the file"),
            TodoFileError::Modify(_) => write!(f,"Error while modifying the file"),
            TodoFileError::NotAPriority(_) => write!(f,"This is not a priority : you should choose between : High, Medium or Low"),
        }
    } 
}