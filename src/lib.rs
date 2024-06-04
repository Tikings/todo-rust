
use chrono::prelude::*;
use std::fmt;

// ____________________ Error types ____________________

#[derive(Debug, PartialEq, PartialOrd)]
pub enum CreationError {
    EmptyString,
    DateError,
} 

// ____________________ Structs and enums ____________________

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Priority {
    High, 
    Medium, 
    Low, 
}
#[derive(PartialEq, Debug)]
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

pub struct TodoList {
    pub list : Vec<TodoElement>, 
    pub path : String, 
    pub path_backup : String,
}