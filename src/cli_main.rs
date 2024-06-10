
// Script to implement the Command line parser using clap crate : 


use clap::{ value_parser, Arg, ArgAction, Command}; 

pub fn generate_command() -> Command {

    // Setting all the arguments to generate the function 

    // Commands that take an argument
    let add_arg = Arg::new("add")
        .short('a')
        .long("add")
        .action(ArgAction::Set)
        .value_name("TASK")
        .conflicts_with_all(["done","delete"])
        .value_parser(value_parser!(String))
        .help("Add a task to the to-do list.");

    let done_arg = Arg::new("done")
        .short('d')
        .long("done")
        .action(ArgAction::Set)
        .value_name("TASK ID")
        .value_parser(value_parser!(String))
        .help("Mark a task of the to-do list as done.");

    let delete_arg = Arg::new("delete")
        .long("delete")
        .action(ArgAction::Set)
        .value_name("TASK ID")
        .value_parser(value_parser!(String))
        .help("Delete the task from the to-do list");

    //Commands that don't takes any arguments
    let restore_arg = Arg::new("restore")
        .short('u')
        .long("undo")
        .exclusive(true)
        .action(ArgAction::SetTrue)
        .help("Undo changes that have been done.");

    let sort_arg = Arg::new("sort")
        .short('s')
        .long("sort")
        .action(ArgAction::SetTrue)
        .conflicts_with("list")
        .help("Sort the tasks by priority. From HIGH to LOW.");

    let reset_arg = Arg::new("reset")
        .short('r')
        .long("reset")
        .action(ArgAction::SetTrue)
        .exclusive(true)
        .help("Reset the to-do list");
    
    let list_arg = Arg::new("list")
        .short('l')
        .long("list")
        .action(ArgAction::SetTrue)
        .help("List all the elements of the to-do list (Done and Undone).");


    // Command parser : 
    Command::new("To-do List")
        .author("Tikings, tikings.zechnas@gmail.com")
        .version("1.0.0")
        .about("A Todo list CLI tool to keep track of things that has to be done.")
        .arg_required_else_help(true)
        .arg(add_arg)
        .arg(done_arg)
        .arg(restore_arg)
        .arg(sort_arg)
        .arg(reset_arg)
        .arg(delete_arg)
        .arg(list_arg)

}
