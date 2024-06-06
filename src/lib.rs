
use chrono::prelude::*;
use std::{fmt::{self, Display},
     fs:: {self, metadata, DirBuilder, File, OpenOptions},
     io::BufWriter,
};
use serde_json;
use serde::{self, ser::SerializeStruct, Serialize};

// ____________________ Guide lines ____________________

//TODO : Fix the path management in the file creation (to be able to use this CLI tool on windows)

// To save the files : 
// Option 1 : Serialise and desirialize all the list -> Do modification on the objects on rust 
// It will be easier to have the different versions of the todo to allow and undo of the list


// ____________________ Error types ____________________

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
}

// ____________________ Structs and enums ____________________

#[derive(PartialEq, PartialOrd, Debug, Serialize)]
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

impl serde::Serialize for TodoElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let mut s = serializer.serialize_struct("TodoElement", 4)?;
                s.serialize_field("content", &self.content)?;
                s.serialize_field("priority", &self.priority)?;
                s.serialize_field("status", &self.status)?;
                s.serialize_field("created", &self.created)?;
                s.end()
    }
}

#[derive(Debug, PartialEq)]
pub struct TodoList {
    pub list : Vec<TodoElement>, 
    pub path : String, 
    pub path_backup : String,
}

impl TodoList {

    fn check_todo_dir(path : &str) -> Result<String, CreationError> {
        // Creating the folder that will contain the current data and the backup data.
        let mut path_dir = ".todo".to_string();
        path_dir = format!("{}/{}",path, path_dir);
        let dir_result = metadata(&path_dir);

        let dir_exist = match dir_result {
            Ok(meta) => meta.is_dir(),
            Err(_) => false,
        };

        if !dir_exist {
            let build_dir = DirBuilder::new().create(&path_dir);
            match build_dir {
                Ok(_) => println!(".todo directory created !"),
                Err(_) => return Err(CreationError::FolderErr),
            }
        }
        println!("Path created : {}",path_dir);
        Ok(path_dir)
    }

    // Check if the file where we store the information of the todo exists and if not create one.
    fn check_save_todo_file(dir_path : &str) -> Result<File,CreationError>{

        let path = format!("{}/save.todo", dir_path);

        let res  = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path);

        match res {
            Ok(file) => Ok(file),
            Err(_) =>  Err(CreationError::FileCreation),
        }
    }

    fn check_backup_todo_file(dir_path : &str) -> Result<File,CreationError>{

        let path = format!("{}/backup.todo", dir_path);

        let res = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path);

        match res {
            Ok(file) => Ok(file),
            Err(_) =>  Err(CreationError::FileCreation),
        }
    }

    // Create a new todo-list
    pub fn new(dir_path : &str) -> Result<Self, CreationError> {
        
        // Empty vector that will contain all the elements of the todo listo
        let list : Vec<TodoElement> = Vec::new();

        let path_todo : String;
        // Check if the directory to save the todo files exists and if it is not the case try to create one
        let dir_save = Self::check_todo_dir(dir_path);
        match dir_save {
            Ok(s) => {
                println!("Directory created");
                path_todo = s.clone();
            }
            Err(e) => return Err(e),
        }

        // Create the save file if it doesn't exists
        let save_file = Self::check_save_todo_file(&path_todo);
        match save_file {
            Ok(_) => println!("Save file for todo list created !"),
            Err(e) => return Err(e),
        }

        // Create the backup file if it doesn't exists
        let backup_file = Self::check_backup_todo_file(&path_todo);
        match backup_file {
            Ok(_) => println!("Backup file for todo list created !"),
            Err(e) => return Err(e),
        }

        Ok(TodoList{
            list : list,
            path : format!("{}/save.todo",path_todo),
            path_backup : format!("{}/backup.todo",path_todo),
        })
    }


    pub fn add(&self, args : &[String]) {
        todo!();
    }

    pub fn remove(&self, args : &[String]) {
        todo!();
    }


    pub fn done(&self, args : &[String]) {
        todo!();
    }

    pub fn sort(&self) {
        todo!();
    }


    pub fn reset(&self) {
        todo!();
    }

    pub fn restore(&self) {
        todo!();
    }

    pub fn write_file(&self) -> Result<(),TodoFileError>{
        // Function that write the content of the todo list to the file ih the self.path location

        let save_file : File;
        match OpenOptions::new()
        .write(true)
        .open(&self.path) {
            Ok(f) => save_file = f,
            Err(e) => return Err(TodoFileError::OpenFile(e)),
        }

        let buffer = BufWriter::new(&save_file);

        match serde_json::to_writer(buffer, &self) {
            Ok(_) => (),
            Err(e) => return Err(TodoFileError::WriteError(e)),
        }

        Ok(())
        
    }

    pub fn backup_data(&self) -> Result<(), TodoFileError> {

        //Opening backup file
        let backup_file : File; 

        let result_backup_file = OpenOptions::new()
        .write(true)
        .open(&self.path_backup);
    
        match result_backup_file {
            Ok(f) => backup_file = f,
            Err(e) => return Err(TodoFileError::OpenFile(e)),
        };

        // Clearing the back_up file
        match backup_file.set_len(0) {
            Ok(_) => (),
            Err(_) => return Err(TodoFileError::ClearingError),
        };

        // Copying file content to the other file
        match fs::copy(&self.path,&self.path_backup) {
            Ok(_) => (),
            Err(_) => return Err(TodoFileError::CopyError),
        }

        Ok(())
    }
}

impl serde::Serialize for TodoList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let mut s = serializer.serialize_struct("TodoList", 1)?;
                s.serialize_field("list", &self.list)?;
                s.end()
    }
}