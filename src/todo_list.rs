// * Importation of the modules

use super::todo_element::{TodoElement,Priority};
use super::errors::*;
use std::path::PathBuf;
use std::{
     fs:: {self, metadata, DirBuilder, File, OpenOptions},
     io::{self, BufReader, BufWriter, Write }, 
};
use colored::Colorize;

use serde_json;
use serde::{self, Deserialize, Serialize};

// * Structs and enums 

#[derive(Debug, PartialEq, Serialize,Deserialize)]
pub struct TodoList {
    pub list : Vec<TodoElement>, 
    pub path : String, 
    pub path_backup : String,
    pub hash_list : Vec<String>,
}

impl TodoList {

    // * Functions for file management

    /// Creating the folder that will contain the current data and the backup data.
    fn check_todo_dir(path : &PathBuf) -> Result<PathBuf, CreationError> {

        let mut path_dir = path.clone();
        // The folder olding the saving and backup files will be named as following 
        path_dir.push(".todo");

        // Checking if this dir already exists 
        let dir_result = metadata(&path_dir);

        let dir_exist = match dir_result {
            Ok(meta) => meta.is_dir(),
            Err(_) => false,
        };

        // If its not the case, create the directory
        if !dir_exist {
            let build_dir = DirBuilder::new().create(&path_dir);
            match build_dir {
                Ok(_) => println!(".todo directory created !"),
                Err(_) => return Err(CreationError::FolderErr),
            }
        }
        Ok(path_dir)
    }

    // Check if the file where we store the information of the todo exists and if not create one.
    fn check_save_todo_file(path : &PathBuf) -> Result<File,CreationError>{

        // Path of the save file
        let mut path_dir = path.clone();
        path_dir.push("save");
        path_dir.set_extension("todo");


        let res  = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_dir);

        match res {
            Ok(file) => Ok(file),
            Err(e) =>  {
                println!("{}", e);
                Err(CreationError::FileCreation)
            }
        }
    }

    // Check if the backup file exists and creates it if it isn't the case
    fn check_backup_todo_file(path : &PathBuf) -> Result<File,CreationError>{

        // Path of the backup file
        let mut path_dir = path.clone();
        path_dir.push("backup");
        path_dir.set_extension("todo");

        let res  = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_dir);

        match res {
            Ok(file) => Ok(file),
            Err(_) =>  Err(CreationError::FileCreation),
        }

    }

    pub fn write_file(&self) -> Result<(),TodoFileError>{
        // Function that save the content of the todo list struct to the file in the self.path location

        // Opening the file
        let save_file : File;
        match OpenOptions::new()
        .write(true)
        .open(&self.path) {
            Ok(f) => save_file = f,
            Err(e) => return Err(TodoFileError::OpenFile(e)),
        }

        // Clearing the save file
        match save_file.set_len(0) {
            Ok(_) => (),
            Err(_) => return Err(TodoFileError::ClearingError),
        };

        let buffer = BufWriter::new(&save_file);

        // Saving the data formatted as a json string 
        match serde_json::to_writer(buffer, &self) {
            Ok(_) => (),
            Err(e) => return Err(TodoFileError::WriteError(e)),
        }

        Ok(())
        
    }

    pub fn backup_data(&self) -> Result<(), TodoFileError> {
        // Back up the data to a "backup.todo" file.

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

    pub fn from_data(path : &PathBuf) -> Result<Self, TodoFileError> {
        // Function that retrieve the todo list from the json formated data of the save.todo file

        let save_file : File;
        match OpenOptions::new()
        .read(true)
        .open(path) {
            Ok(f) => save_file = f,
            Err(e) => return Err(TodoFileError::OpenFile(e)),
        }

        let buf  = BufReader::new(save_file);
        let todo= serde_json::from_reader(buf).unwrap();
        
        Ok(todo)

    }

    // * Methods for functionalities

    // Create a new todo-list
    pub fn new(dir_path : &PathBuf) -> Result<Self, CreationError> {
        
        // Empty vector that will contain all the elements of the todo listo
        let list : Vec<TodoElement> = Vec::new();
        let hash_list : Vec<String> = Vec::new();

        let path_todo ;
        // Check if the directory to save the todo files exists and if it is not the case try to create one
        let dir_save = Self::check_todo_dir(&dir_path);
        match dir_save {
            Ok(s) => {
                println!("Directory created");
                path_todo = s;
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
        
        let path_as_str = path_todo.into_os_string().into_string().unwrap();

        Ok(TodoList{
            list : list,
            path : format!("{}/save.todo",path_as_str),
            path_backup : format!("{}/backup.todo",path_as_str),
            hash_list : hash_list,
        })
    }


    pub fn add(&mut self, content : String , priority : String) -> Result<(), TodoFileError> {

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Adding a new TodoElementTo the list from the arguments

        let lower_priority = priority.to_lowercase();
        let parsed_priority : Priority= match lower_priority.as_str() {
            "high" | "h" => Priority::High ,
            "low" | "l" => Priority::Low ,
            "medium" | "m" => Priority::Medium ,
            _ => return Err(TodoFileError::NotAPriority(lower_priority)),
        };

        let element_to_add = TodoElement::new(content, parsed_priority).unwrap();

        self.list.push(element_to_add);
        
        Ok(())
        
    }

    pub fn remove(&mut self,  index : usize) -> Result<(), TodoFileError> {

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        
        // Retriving the hash corresponding to the task to remove
        let hash = self.hash_list[index].clone();
        let mut index_to_remove : usize = self.hash_list.len() + 1 ; 

        for (ind, task) in self.list.iter().enumerate() {
          if hash == task.hash {
            index_to_remove = ind;
          } 
        }

        if index_to_remove == self.hash_list.len() {
            return Err(TodoFileError::Modify("Index does not exist".to_string()));
        }

        self.list.remove(index_to_remove);

        Ok(())
    }


    pub fn done(&mut self,  index : usize) -> Result<(), TodoFileError>  {

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        
        // Retrieving the hash corresponding to the task to remove
        let hash = self.hash_list[index].clone();
        let mut index_to_change : usize = self.hash_list.len() + 1 ; 

        for (ind, task) in self.list.iter().enumerate() {
          if hash == task.hash {
            index_to_change = ind;
          } 
        }

        if index_to_change == self.hash_list.len() {
            return Err(TodoFileError::Modify("Index does not exist".to_string()));
        }

        // Setting the element status as done
        self.list[index_to_change].status = true;

        Ok(())
    }



    pub fn reset(&mut self) -> Result<(), TodoFileError> {
        // Reset the todo list by removing the data from the save.todo file.

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        //Opening file
        let save_file : File; 

        let result_save_file = OpenOptions::new()
        .write(true)
        .open(&self.path);
    
        match result_save_file {
            Ok(f) => save_file = f,
            Err(e) => return Err(TodoFileError::OpenFile(e)),
        };

        // Clearing the save file
        match save_file.set_len(0) {
            Ok(_) => (),
            Err(_) => return Err(TodoFileError::ClearingError),
        };

        self.list = Vec::new();

        match self.write_file() {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub fn restore(&self) -> Result<(),TodoFileError> {
        // Restore the previous todo list from the backup file by copying its content to the save file.

        //Opening backup file
        let save_file : File; 

        let result_save_file = OpenOptions::new()
        .write(true)
        .open(&self.path);
    
        match result_save_file {
            Ok(f) => save_file = f,
            Err(e) => return Err(TodoFileError::OpenFile(e)),
        };

        // Clearing the save file
        match save_file.set_len(0) {
            Ok(_) => (),
            Err(_) => return Err(TodoFileError::ClearingError),
        };

        // Copying file content to the other file
        match fs::copy(&self.path_backup,&self.path) {
            Ok(_) => (),
            Err(_) => return Err(TodoFileError::CopyError),
        }

        Ok(())
    }

    // * Functions to display the todo list

    pub fn display_by_date(&mut self) -> Result<(), TodoFileError>  {

        // Creating the counter for to display the data
        let mut counter = 0;

        // Creating the new hash list
        let mut hash_list : Vec<String> = Vec::new();
        
        // Buffer to print in the terminal the data
        let stdout = io::stdout();
        let mut buf = BufWriter::new(stdout);
        
        // To store the formatted elements
        let mut data_undone = String::new();
        let mut data_done = String::new();

        // Done tasks
        let mut done_tasks : Vec<&TodoElement> = Vec::new(); 
        // Undone tasks 
        let mut undone_tasks : Vec<&TodoElement> = Vec::new();
        
        // Sorting the different task in the 2 lists
        for todo in self.list.iter() {
            if todo.status {
                done_tasks.push(todo); 
            } else {
                undone_tasks.push(todo);
            }
        }

        // Sorting the undone tasks by date
        undone_tasks.sort_by(|a ,b| a.created.cmp(&b.created));
        
        // Formatting the data to display 
        for task in undone_tasks.iter() {
            data_undone = format!("{} \n {}. {}", data_undone, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        // Creating the string for the 
        for task in done_tasks.iter() {
            data_done = format!("{} \n {}. {}", data_done, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        // Displaying the data
        let data = format!(
r#"
{}

{}

{}

{}

"#,
 "TO-DO".truecolor(199, 86, 30).bold(),
data_undone,
 "DONE".truecolor(35, 223, 35).bold(),
data_done
);

        buf.write_all(data.as_bytes()).expect("Failed to write to the buf writer");

        self.hash_list = hash_list;

        Ok(())
    }


    pub fn display_by_priority(&mut self) -> Result<(), TodoFileError>  {

        // Creating the counter for to display the data
        let mut counter = 0;

        // Creating the new hash list
        let mut hash_list : Vec<String> = Vec::new();
        
        // Buffer to print in the terminal the data
        let stdout = io::stdout();
        let mut buf = BufWriter::new(stdout);
        
        // Data to display in the terminal
        let mut data_high : String = String::new();
        let mut data_med : String = String::new();
        let mut data_low : String = String::new();
        let mut data_done : String = String::new();

        // splitting the done tasks and undone task in 2 Lists 
        let mut done_tasks : Vec<&TodoElement> = Vec::new(); 
        let mut undone_tasks : Vec<&TodoElement> = Vec::new();
        
        // Sorting the different task in the 2 lists
        for todo in self.list.iter() {
            if todo.status {
                done_tasks.push(todo); 
            } else {
                undone_tasks.push(todo);
            }
        }

        // Sorting the undone tasks by their priority
        let mut high_priority : Vec<&TodoElement> = Vec::new();
        let mut med_priority : Vec<&TodoElement> = Vec::new();
        let mut low_priority : Vec<&TodoElement> = Vec::new();

    for task in undone_tasks.iter(){
            if task.priority == Priority::High {
                high_priority.push(task);
            }
            else if task.priority == Priority::Medium {
                med_priority.push(task)
            }
            else {
                low_priority.push(task)
            }
        }

        // Sorting the undone tasks by date
        high_priority.sort_by(|a ,b| a.created.cmp(&b.created));
        med_priority.sort_by(|a ,b| a.created.cmp(&b.created));
        low_priority.sort_by(|a ,b| a.created.cmp(&b.created));
        
        // Formatting the data to display 
        for task in high_priority.iter() {
            data_high = format!("{} \n {}. {}", data_high, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        for task in med_priority.iter() {
            data_med = format!("{} \n {}. {}", data_med, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        for task in low_priority.iter() {
            data_low = format!("{} \n {}. {}", data_low, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }
        
        for task in done_tasks.iter() {
            data_done = format!("{} \n {}. {}", data_done, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        let data = format!(
r#"
{}

{}

{}

{}

{}

{}

{}

{}

{}

"#,
        "______ TO-DO ______".truecolor(199, 86, 30).bold(),
        "------ High priority ------ ".bright_red().bold(),
        data_high,
        "------ Medium priority ------".bright_yellow().bold(),
        data_med,
        "------ Low priority ------".bright_cyan().bold(),
        data_low,
        "DONE".truecolor(35, 223, 35).bold(),
        data_done
        );
        // Displaying the data
        buf.write_all(data.as_bytes()).expect("Failed to write to the buf writer");

        self.hash_list = hash_list;

        Ok(())    }

}
