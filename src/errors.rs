// * Error types 

use std::fmt::{self,Display};
use serde_json;

/// Different errors encountered while creating the TodoList and the TodoElements 
#[derive(Debug, PartialEq, PartialOrd)]
pub enum CreationError {
    /// String is empty while creating a Todo element
    EmptyString,

    /// Failed to create the folder
    FolderErr,

    /// Failed to create the save or the back-up file
    FileCreation,
} 

impl Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            CreationError::EmptyString => write!(f,"Empty string"),
            CreationError::FolderErr => write!(f,"Directory error"),
            CreationError::FileCreation => write!(f,"File creation"),
        }
    } 
}

/// Different errors that can be encountered while modifying, saving or doing a backup of the TodoList
#[derive(Debug)]
pub enum TodoFileError {
    /// Failed to open a file
    OpenFile(std::io::Error),

    /// Error while clearing the file
    ClearingError,

    /// Error while doing the backup of the save file
    CopyError,

    /// Error while writing on the save file
    WriteError(serde_json::Error),

    /// Error while modifying the todo list
    Modify(String),

    /// The parsed input from the add subcommand is not a accepted. It should be : high, medium, low, h, m or l. 
    NotAPriority(String),
}

impl Display for TodoFileError {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            TodoFileError::OpenFile(_) => write!(f, "Error while opening the save file "),
            TodoFileError::ClearingError => write!(f,"Error while clearing the file."),
            TodoFileError::CopyError => write!(f,"Error while copying the file"),
            TodoFileError::WriteError(_) => write!(f,"Error while parsing the file"),
            TodoFileError::Modify(_) => write!(f,"Error while modifying the file"),
            TodoFileError::NotAPriority(_) => write!(f,"This is not a priority : you should choose between : High, Medium or Low"),
        }
    } 
}