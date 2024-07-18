// * Importation of the modules 

use chrono::prelude::*;
use std::fmt;
use serde::{self, Deserialize, Serialize};
use super::errors::CreationError;

// * Priority Enum 

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize, Ord, Eq)]
pub enum Priority {
    High, 
    Medium, 
    Low, 
}

// * Todo Element

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct TodoElement {
    pub content : String,
    pub priority : Priority,
    pub status : bool,
    pub created : String,
}


impl TodoElement {
    pub fn new(s : String, prio : Priority) -> Result<Self, CreationError> {
        // Retrieving the date of creation of the todo element
        let date = Local::now().date_naive();
        let created = date.format("%d-%m-%Y").to_string();
        
        // String empty error
        if s.len() == 0 {
            return Err(CreationError::EmptyString);
        }

        Ok(TodoElement {
            content :  s,
            priority : prio,
            status : false,
            created : created,
        })
    }
}

impl fmt::Display for TodoElement {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let checkbox : &str; 

        if self.status {
            checkbox = "[*]";
        } else {
            checkbox = "[ ]";
        }

        write!(f, "{} {} | {}",checkbox, self.content,self.created)
    }
}
