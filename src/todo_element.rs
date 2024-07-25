// * Importation of the modules 

use colored::{ColoredString, Colorize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::prelude::*;
use std::fmt;
use serde::{self, Deserialize, Serialize};
use super::errors::CreationError;

// * Hash generator

pub fn generate_hash() -> String {
    // Function that generate a 32 char length String to be used as a hash in the TodoElement generation to make the difference between the element
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    rand_string 
}


// * Priority Enum 

// The different priorities that can be set for a task / element 
#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize, Ord, Eq)]
pub enum Priority {
    High, 
    Medium, 
    Low, 
}

// * Todo Element

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct TodoElement {
    // Content of the task 
    pub content : String,

    // Priority of the task -> medium by default
    pub priority : Priority,

    // Done -> True, Undone -> False 
    pub status : bool,

    // Date of creation of the element
    pub created : String,

    // Unique ID of the task in the todo list.
    pub hash : String, 
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
        let hash = generate_hash();


        Ok(TodoElement {
            content :  s,
            priority : prio,
            status : false,
            created : created,
            hash : hash ,
        })
    }
}

impl fmt::Display for TodoElement {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {

        let checkbox : ColoredString;
        let formatted_content : ColoredString;
        
        //Done tasks
        if self.status {
            checkbox = "[*]".bold();
            formatted_content = self.content.strikethrough();

        } else { //Undone
            checkbox = "[ ]".bold();
            formatted_content = self.content.normal();
        }

        let output = format!("{} {} | {}",checkbox, formatted_content,self.created.italic());


        write!(f, "{}", output )
    }
}
