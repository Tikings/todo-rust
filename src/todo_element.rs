// * Importation of the modules 

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
        let checkbox : &str; 

        if self.status {
            checkbox = "[*]";
        } else {
            checkbox = "[ ]";
        }

        write!(f, "{} {} | {}",checkbox, self.content,self.created)
    }
}
