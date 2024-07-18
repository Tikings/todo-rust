

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

pub mod todo_element;
pub mod todo_list;
pub mod errors;