// use todo::{TodoElement, TodoList, Priority};
// use std::env::args;
use todo::cli_main;

fn main() {

    // let args : Vec<String> = args().collect();
    // println!("{:?}", &args[1..]);

    let cmd = cli_main::generate_command();
    // let matches = cmd.try_get_matches_from(&args[1..]).unwrap_or_else(|e| {
    //     e.exit();
    // });

    let matches = cmd.get_matches(); 

    println!("matches : {:?}", matches);

    let add_args : &String = matches.get_one("delete").unwrap();
    
    println!("{:?}", add_args);
    
}

