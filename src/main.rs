
use todo::{cli::cli, errors::TodoFileError, todo_list::TodoList};


fn main() {

    // * Retrieve or create the todo list

    // Retrieve the current dir   
    let current_dir  = std::env::current_dir().unwrap();

    // path for the save file
    let mut save_path = current_dir.clone();
    save_path.push(".todo");
    save_path.push("save");
    save_path.set_extension("todo");

    // Trying to retrieve the data from the save file 
    let result_todo : Result<TodoList,TodoFileError> = TodoList::from_data(&save_path);

    let mut todo_list = match result_todo {
        Ok(todo) => todo, //If the save file already exists 
        Err(_) => {
            let todo = TodoList::new(&current_dir).unwrap();
            todo.write_file().unwrap();
            todo
        } // If it doesn't exist, we create the todo list 
    };
    
   
    // * Operation of the to-do list 

    let command = cli();
    // Getting the matches from the parser
    let matches  = command.get_matches();

    // Getting the subcommand used and its matches
    let submatches = matches.subcommand();

    match submatches {
        Some(("add", matches)) => {

            let content : String = matches.get_one::<String>("task").expect("Please prompt a task").clone();
            let priority : String = matches.get_one::<String>("priority").expect("Please prompt a task").clone();

            match &todo_list.add(content, priority) {
                Ok(_) => println!("Added !"),
                Err(e) => println!("{}", e)
            };

            match &todo_list.display_by_date() {
                Ok(_) => (),
                Err(e) => println!("Unable to display : {}",e),
            }

            // Saving the changes
            match &todo_list.write_file() {
                Ok(_) => (),
                Err(e) => println!("Error while saving the file : {}", e)
            }


        } 
        Some(("done", matches)) => {
            let index : usize = matches.get_one::<usize>("id").expect("An ID is required").clone();
            
            match &todo_list.done(index) {
                Ok(_) => println!("Set as done !"),
                Err(e) => println!("{}", e)
            }

            match &todo_list.display_by_date() {
                Ok(_) => (),
                Err(e) => println!("Unable to display : {}",e),
            }

            // Saving the changes
            match &todo_list.write_file() {
                Ok(_) => (),
                Err(e) => println!("Error while saving the file : {}", e)
            }


        }
        Some(("remove", matches)) => {
            let index : usize = matches.get_one::<usize>("id").expect("An ID is required").clone();
            
            match &todo_list.remove(index) {
                Ok(_) => println!("Remove task #{}",index),
                Err(e) => println!("{}", e)
            }

            match &todo_list.display_by_date() {
                Ok(_) => (),
                Err(e) => println!("Unable to display : {}",e),
            }

            // Saving the changes
            match &todo_list.write_file() {
                Ok(_) => (),
                Err(e) => println!("Error while saving the file : {}", e)
            }


        }
        Some(("reset", _matches)) => {
            match &todo_list.reset() {
                Ok(_) => println!("To-do list got reset"),
                Err(e) => println!("{}", e),
            }

            match &todo_list.display_by_date() {
                Ok(_) => (),
                Err(e) => println!("Unable to display : {}",e),
            }

        }
        Some(("restore", _matches)) => {
            match &todo_list.restore() {
                Ok(_) => println!("Restored last version of the to-do list"),
                Err(e) => println!("{}", e),
            }

        }
        Some(("sort", _matches)) => {
            match &todo_list.display_by_priority() {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        _ => {
            match &todo_list.display_by_date() {
                Ok(_) => (),
                Err(e) => println!("Unable to display : {}",e),
            }

        }

    }

}

