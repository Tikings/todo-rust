
// use chrono::prelude::*;
// use std::{fmt::{self, Display},
//      fs:: {self, metadata, DirBuilder, File, OpenOptions},
//      io::{self, BufReader, BufWriter, Write }, 
// };

// use serde_json;
// use serde::{self, Deserialize, Serialize};


pub mod todo_element;
pub mod todo_list;
pub mod errors;


// ____________________ Guide lines ____________________

//TODO : Fix the path management in the file creation (to be able to use this CLI tool on windows)

// To save the files : 
// Option 1 : Serialise and desirialize all the list -> Do modification on the objects on rust 
// It will be easier to have the different versions of the todo to allow and undo of the list

// Make a clear file function taking a path as an argument
// Change the access of the backup and write functions

// TODO : Change the display function for the TodoList 

//TODO : Find a way to sort the element before displaying them. 
// The different type of sort : 
// - Default : Date
// - By priority

// ____________________ Error types ____________________

// #[derive(Debug, PartialEq, PartialOrd)]
// pub enum CreationError {
//     EmptyString,
//     DateError,
//     FolderErr,
//     FileCreation,
// } 

// impl Display for CreationError {
//     fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
//         match *self {
//             CreationError::EmptyString => write!(f,"Empty string"),
//             CreationError::DateError => write!(f,"Date Error"),
//             CreationError::FolderErr => write!(f,"Directory error"),
//             CreationError::FileCreation => write!(f,"File creation"),
//         }
//     } 
// }

// #[derive(Debug)]
// pub enum TodoFileError {
//     OpenFile(std::io::Error),
//     ClearingError,
//     CopyError,
//     WriteError(serde_json::Error),
//     Modify(String)
// }

// // ____________________ Structs and enums ____________________

// #[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize, Ord, Eq)]
// pub enum Priority {
//     High, 
//     Medium, 
//     Low, 
// }

// #[derive(PartialEq, Debug, Deserialize, Serialize)]
// pub struct TodoElement {
//     pub content : String,
//     pub priority : Priority,
//     pub status : bool,
//     pub created : String,
// }

// impl TodoElement {
//     pub fn new(s : String, prio : Priority) -> Result<Self, CreationError> {
//         // Retrieving the date of creation of the todo element
//         let date = Local::now().date_naive();
//         let created = date.format("%d-%m-%Y").to_string();
        
//         // String empty error
//         if s.len() == 0 {
//             return Err(CreationError::EmptyString);
//         }

//         Ok(TodoElement {
//             content :  s,
//             priority : prio,
//             status : false,
//             created : created,
//         })
//     }
// }

// impl fmt::Display for TodoElement {
//     fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
//         let checkbox : &str; 

//         if self.status {
//             checkbox = "[*]";
//         } else {
//             checkbox = "[ ]";
//         }

//         write!(f, "{} {} | {}",checkbox, self.content,self.created)
//     }
// }


// // #[derive(Debug, PartialEq, Deserialize)]
// #[derive(Debug, PartialEq, Serialize,Deserialize)]
// pub struct TodoList {
//     pub list : Vec<TodoElement>, 
//     pub path : String, 
//     pub path_backup : String,
// }

// impl TodoList {

//     fn check_todo_dir(path : &str) -> Result<String, CreationError> {
//         // Creating the folder that will contain the current data and the backup data.
//         let mut path_dir = ".todo".to_string();
//         path_dir = format!("{}/{}",path, path_dir);
//         let dir_result = metadata(&path_dir);

//         let dir_exist = match dir_result {
//             Ok(meta) => meta.is_dir(),
//             Err(_) => false,
//         };

//         if !dir_exist {
//             let build_dir = DirBuilder::new().create(&path_dir);
//             match build_dir {
//                 Ok(_) => println!(".todo directory created !"),
//                 Err(_) => return Err(CreationError::FolderErr),
//             }
//         }
//         println!("Path created : {}",path_dir);
//         Ok(path_dir)
//     }

//     // Check if the file where we store the information of the todo exists and if not create one.
//     fn check_save_todo_file(dir_path : &str) -> Result<File,CreationError>{

//         let path = format!("{}/save.todo", dir_path);

//         let res  = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(path);

//         match res {
//             Ok(file) => Ok(file),
//             Err(_) =>  Err(CreationError::FileCreation),
//         }
//     }

//     fn check_backup_todo_file(dir_path : &str) -> Result<File,CreationError>{

//         let path = format!("{}/backup.todo", dir_path);

//         let res = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .create(true)
//         .open(path);

//         match res {
//             Ok(file) => Ok(file),
//             Err(_) =>  Err(CreationError::FileCreation),
//         }
//     }

//     // Create a new todo-list
//     pub fn new(dir_path : &str) -> Result<Self, CreationError> {
        
//         // Empty vector that will contain all the elements of the todo listo
//         let list : Vec<TodoElement> = Vec::new();

//         let path_todo : String;
//         // Check if the directory to save the todo files exists and if it is not the case try to create one
//         let dir_save = Self::check_todo_dir(dir_path);
//         match dir_save {
//             Ok(s) => {
//                 println!("Directory created");
//                 path_todo = s.clone();
//             }
//             Err(e) => return Err(e),
//         }

//         // Create the save file if it doesn't exists
//         let save_file = Self::check_save_todo_file(&path_todo);
//         match save_file {
//             Ok(_) => println!("Save file for todo list created !"),
//             Err(e) => return Err(e),
//         }

//         // Create the backup file if it doesn't exists
//         let backup_file = Self::check_backup_todo_file(&path_todo);
//         match backup_file {
//             Ok(_) => println!("Backup file for todo list created !"),
//             Err(e) => return Err(e),
//         }

//         Ok(TodoList{
//             list : list,
//             path : format!("{}/save.todo",path_todo),
//             path_backup : format!("{}/backup.todo",path_todo),
//         })
//     }


//     pub fn add(mut self, content : String , priority : String) -> Result<(), TodoFileError> {

//         // Backup data before reset
//         match self.backup_data() {
//             Ok(_) => (),
//             Err(e) => return Err(e),
//         }

//         // Adding a new TodoElementTo the list from the arguments
//         // Arguments are going to look like : todo add "Task1" -p m -> Add the Task 1 of priority medium ( -p is optional here -> Default : medium)

//         let lower_priority = priority.to_lowercase();
//         let parsed_priority : Priority= match lower_priority.as_str() {
//             "high" | "h" => Priority::High ,
//             "low" | "l" => Priority::Low ,
//             _ => Priority::Medium,
//         };

//         let element_to_add = TodoElement::new(content, parsed_priority).unwrap();

//         self.list.push(element_to_add);
        
//         Ok(())
        
//     }

//     pub fn remove(&mut self,  index : usize) -> Result<(), TodoFileError> {

//         // Backup data before reset
//         match self.backup_data() {
//             Ok(_) => (),
//             Err(e) => return Err(e),
//         }
        
//         self.list.remove(index);
        

//         Ok(())
//     }


//     pub fn done(&mut self,  index : usize) -> Result<(), TodoFileError>  {

//         // Backup data before reset
//         match self.backup_data() {
//             Ok(_) => (),
//             Err(e) => return Err(e),
//         }

//         self.list[index].status = true;

//         Ok(())
//     }

//     pub fn display_by_date(&self) -> Result<(), TodoFileError>  {

//         // Backup data before reset
//         match self.backup_data() {
//             Ok(_) => (),
//             Err(e) => return Err(e),
//         }
        
//         let stdout = io::stdout();
//         let mut buf = BufWriter::new(stdout);
        
//         // Data to display in the terminal
//         let mut data : String = String::from(
//         "TO-DO _____\n"
//         );

//         // Done tasks
//         let mut done_tasks : Vec<&TodoElement> = Vec::new(); 
//         // Undone tasks 
//         let mut undone_tasks : Vec<&TodoElement> = Vec::new();
        
//         // Sorting the different task in the 2 lists
//         for todo in self.list.iter() {
//             if todo.status {
//                 done_tasks.push(todo); 
//             } else {
//                 undone_tasks.push(todo);
//             }
//         }

//         // Sorting the undone tasks by date
//         undone_tasks.sort_by(|a ,b| a.created.cmp(&b.created));
        
//         for task in undone_tasks.iter() {
//             data = format!("{} \n {}", data, task)
//         }
        
//         data = format!(" {} \n {}",data ,"\n DONE _____ \n");

//         for task in done_tasks.iter() {
//             data = format!("{} \n {}", data, task)
//         }

//         data = format!(" {} \n {}",data ,"\n");
        

//         buf.write_all(data.as_bytes()).expect("Failed to write to the buf writer");

//         Ok(())
//     }


//     pub fn display_by_priority(&self) -> Result<(), TodoFileError>  {

//         // Backup data before reset
//         match self.backup_data() {
//             Ok(_) => (),
//             Err(e) => return Err(e),
//         }

//         let stdout = io::stdout();
//         let mut buf = BufWriter::new(stdout);
        
//         // Data to display in the terminal
//         let mut data : String = String::from(
//         "TO-DO _____\n"
//         );

//         // Done tasks
//         let mut done_tasks : Vec<&TodoElement> = Vec::new(); 
//         // Undone tasks 
//         let mut undone_tasks : Vec<&TodoElement> = Vec::new();
        
//         // Sorting the different task in the 2 lists
//         for todo in self.list.iter() {
//             if todo.status {
//                 done_tasks.push(todo); 
//             } else {
//                 undone_tasks.push(todo);
//             }
//         }

//         // Sorting the undone tasks by date
//         undone_tasks.sort_by(|a ,b| a.created.cmp(&b.created));
        
//         for task in undone_tasks.iter() {
//             data = format!("{} \n {}", data, task)
//         }
        
//         data = format!(" {} \n {}",data ,"\n DONE _____ \n");

//         for task in done_tasks.iter() {
//             data = format!("{} \n {}", data, task)
//         }

//         data = format!(" {} \n {}",data ,"\n");
        

//         buf.write_all(data.as_bytes()).expect("Failed to write to the buf writer");

//         Ok(())
//     }


//     pub fn reset(&self) -> Result<(), TodoFileError> {
//         // Reset the todo list by removing the data from the save.todo file.

//         // Backup data before reset
//         match self.backup_data() {
//             Ok(_) => (),
//             Err(e) => return Err(e),
//         }

//         //Opening file
//         let save_file : File; 

//         let result_save_file = OpenOptions::new()
//         .write(true)
//         .open(&self.path);
    
//         match result_save_file {
//             Ok(f) => save_file = f,
//             Err(e) => return Err(TodoFileError::OpenFile(e)),
//         };

//         // Clearing the save file
//         match save_file.set_len(0) {
//             Ok(_) => (),
//             Err(_) => return Err(TodoFileError::ClearingError),
//         };

//         Ok(())
//     }

//     pub fn restore(&self) -> Result<(),TodoFileError> {
//         // Restore the previous todo list from the backup file by copying its content to the save file.

//         //Opening backup file
//         let save_file : File; 

//         let result_save_file = OpenOptions::new()
//         .write(true)
//         .open(&self.path);
    
//         match result_save_file {
//             Ok(f) => save_file = f,
//             Err(e) => return Err(TodoFileError::OpenFile(e)),
//         };

//         // Clearing the back_up file
//         match save_file.set_len(0) {
//             Ok(_) => (),
//             Err(_) => return Err(TodoFileError::ClearingError),
//         };

//         // Copying file content to the other file
//         match fs::copy(&self.path_backup,&self.path) {
//             Ok(_) => (),
//             Err(_) => return Err(TodoFileError::CopyError),
//         }

//         Ok(())
//     }

//     pub fn write_file(&self) -> Result<(),TodoFileError>{
//         // Function that write the content of the todo list to the file ih the self.path location

//         let save_file : File;
//         match OpenOptions::new()
//         .write(true)
//         .open(&self.path) {
//             Ok(f) => save_file = f,
//             Err(e) => return Err(TodoFileError::OpenFile(e)),
//         }

//         let buffer = BufWriter::new(&save_file);

//         match serde_json::to_writer(buffer, &self) {
//             Ok(_) => (),
//             Err(e) => return Err(TodoFileError::WriteError(e)),
//         }

//         Ok(())
        
//     }

//     pub fn backup_data(&self) -> Result<(), TodoFileError> {
//         // Back up the data to a "backup.todo" file.

//         //Opening backup file
//         let backup_file : File; 

//         let result_backup_file = OpenOptions::new()
//         .write(true)
//         .open(&self.path_backup);
    
//         match result_backup_file {
//             Ok(f) => backup_file = f,
//             Err(e) => return Err(TodoFileError::OpenFile(e)),
//         };

//         // Clearing the back_up file
//         match backup_file.set_len(0) {
//             Ok(_) => (),
//             Err(_) => return Err(TodoFileError::ClearingError),
//         };

//         // Copying file content to the other file
//         match fs::copy(&self.path,&self.path_backup) {
//             Ok(_) => (),
//             Err(_) => return Err(TodoFileError::CopyError),
//         }

//         Ok(())
//     }

//     pub fn from_data(path : String) -> Result<Self, TodoFileError> {
//         // Function that retrieve the todo list from the json formated data of the save.todo file

//         let save_file : File;
//         match OpenOptions::new()
//         .read(true)
//         .open(path) {
//             Ok(f) => save_file = f,
//             Err(e) => return Err(TodoFileError::OpenFile(e)),
//         }

//         let buf  = BufReader::new(save_file);
//         let todo= serde_json::from_reader(buf).unwrap();
        
//         Ok(todo)

//     }

//     pub fn sort_by_date(&mut self){
//         // Function that sort the list field by date.
//         self.list.sort_by(|a, b| a.created.cmp(&b.created))
//     }

//     pub fn sort_by_priority(&mut self) {
//         self.list.sort_by(|a,b| a.priority.cmp(&b.priority))
//     }

// }

// impl fmt::Display for TodoList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//         let mut output = String::new();
//         for element in self.list.iter() {
//             let to_push = format!("{} \n", element);
//             output.push_str(&to_push);
//         }

//         let write : String = format!("{}", output);
//         write!(f,"{}",write)
//     }
// }