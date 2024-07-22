// * Importation of the modules

use super::todo_element::{TodoElement,Priority};
use super::errors::*;
use std::{fmt,
     fs:: {self, metadata, DirBuilder, File, OpenOptions},
     io::{self, BufReader, BufWriter, Write }, 
};

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

    pub fn from_data(path : String) -> Result<Self, TodoFileError> {
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
    pub fn new(dir_path : &str) -> Result<Self, CreationError> {
        
        // Empty vector that will contain all the elements of the todo listo
        let list : Vec<TodoElement> = Vec::new();
        let hash_list : Vec<String> = Vec::new();

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
            hash_list : hash_list,
        })
    }


    pub fn add(mut self, content : String , priority : String) -> Result<(), TodoFileError> {

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Adding a new TodoElementTo the list from the arguments
        // Arguments are going to look like : todo add "Task1" -p m -> Add the Task 1 of priority medium ( -p is optional here -> Default : medium)

        let lower_priority = priority.to_lowercase();
        let parsed_priority : Priority= match lower_priority.as_str() {
            "high" | "h" => Priority::High ,
            "low" | "l" => Priority::Low ,
            _ => Priority::Medium,
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
        
        // Retriving the hash corresponding to the task to remove
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

        self.list[index_to_change].status = true;

        Ok(())
    }



    pub fn reset(&self) -> Result<(), TodoFileError> {
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

        // Clearing the back_up file
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

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Creating the counter for to display the data
        let mut counter = 0;

        // Creating the new hash list
        let mut hash_list : Vec<String> = Vec::new();
        
        // Buffer to print in the terminal the data
        let stdout = io::stdout();
        let mut buf = BufWriter::new(stdout);
        
        // Data to display in the terminal
        let mut data : String = String::from(
        "TO-DO _____\n"
        );

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
            data = format!("{} \n {}. {}", data, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }
        
        data = format!(" {} \n {}",data ,"\n DONE _____ \n");

        for task in done_tasks.iter() {
            data = format!("{} \n {}. {}", data, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        data = format!(" {} \n {}",data ,"\n");
        // Displaying the data
        buf.write_all(data.as_bytes()).expect("Failed to write to the buf writer");

        self.hash_list = hash_list;

        Ok(())
    }


    pub fn display_by_priority(&mut self) -> Result<(), TodoFileError>  {

        // Backup data before reset
        match self.backup_data() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Creating the counter for to display the data
        let mut counter = 0;

        // Creating the new hash list
        let mut hash_list : Vec<String> = Vec::new();
        
        // Buffer to print in the terminal the data
        let stdout = io::stdout();
        let mut buf = BufWriter::new(stdout);
        
        // Data to display in the terminal
        let mut data : String = String::from(
        "TO-DO _____ \n \n ____ High ____ \n"
        );

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
            data = format!("{} \n {}. {}", data, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }
        
        data = format!("{} \n \n ____ Medium ____ \n", data);

        for task in med_priority.iter() {
            data = format!("{} \n {}. {}", data, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        data = format!("{} \n \n ____ Low ____ \n", data);

        for task in low_priority.iter() {
            data = format!("{} \n {}. {}", data, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }
        
        data = format!(" {} \n {}",data ,"\n DONE _____ \n");

        for task in done_tasks.iter() {
            data = format!("{} \n {}. {}", data, counter, task);
            hash_list.push(task.hash.clone());
            counter += 1 ;
        }

        data = format!(" {} \n {}",data ,"\n");
        // Displaying the data
        buf.write_all(data.as_bytes()).expect("Failed to write to the buf writer");

        self.hash_list = hash_list;

        Ok(())    }

}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut output = String::new();
        for element in self.list.iter() {
            let to_push = format!("{} \n", element);
            output.push_str(&to_push);
        }

        let write : String = format!("{}", output);
        write!(f,"{}",write)
    }
}