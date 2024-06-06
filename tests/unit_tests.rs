use todo::{self, CreationError, Priority, TodoElement, TodoList};
use chrono::prelude::*;
use std::fs:: {read_to_string,OpenOptions}; 


#[test]
fn display_todo_element_true() {

    let element = TodoElement {
        content : "Task 1".to_string(),
        priority : todo::Priority::Low,
        status : true,
        created : "25-06-2024".to_string(),
    };

    assert_eq!("[*] Task 1 | 25-06-2024", format!("{}",element))

}

#[test]
fn display_todo_element_false() {

    let element = todo::TodoElement {
        content : "Task 1".to_string(),
        priority : todo::Priority::Low,
        status : false,
        created : "25-06-2024".to_string(),
    };

    assert_eq!("[ ] Task 1 | 25-06-2024", format!("{}",element))

}

#[test]
fn create_todo_element() {
    let element = TodoElement::new( "Task 1".to_string(),  Priority::Low).unwrap();
    let date = Local::now().date_naive();
    let created = date.format("%d-%m-%Y").to_string();
    assert_eq!(element, TodoElement {
        content : "Task 1".to_string(),
        priority : todo::Priority::Low,
        status : false,
        created : created,
    })

}

#[test]
fn create_todo_element_empty_string() {
    let element = TodoElement::new( "".to_string(),  Priority::Low).unwrap_err();
    assert_eq!(element, CreationError::EmptyString)
}

#[test]
fn new_todo_list() {
    let todo_list = TodoList::new("tests/test_todo").unwrap();
    assert_eq!(todo_list, TodoList {
        list : Vec::new(),
        path : "tests/test_todo/.todo/save.todo".to_string(),
        path_backup : "tests/test_todo/.todo/backup.todo".to_string(),
    })
}

#[test]
fn save_data_todo_list() {
    //  Clean the file before use
    let path = "tests/test_todo/.todo/save.todo".to_string();

    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(&path).unwrap();

    // Clearing the back_up file
    match file.set_len(0) {
        Ok(_) => (),
        Err(e) => println!("Error occured {e}"),
    };

    //Creating an instance of the todo list
    let list_element = vec![
        TodoElement {content  : "Task 1".to_string(), priority :  Priority::High, status : false, created : "26-05-2024".to_string()},
        TodoElement {content  : "Task 2".to_string(), priority :  Priority::High, status : false, created : "23-06-2024".to_string()},
        TodoElement {content  : "Task 3".to_string(), priority :  Priority::High, status : false, created : "22-01-2022".to_string()},
    ];

    let todo_list = TodoList{
        list : list_element,
        path : path.clone(),
        path_backup :  "tests/test_todo/.todo/backup.todo".to_string(),
    };

    todo_list.write_file().expect("Didn't saved the file... ");

    //Check if the file contains the right json string
    let content = read_to_string(&path).unwrap();
    let should_be = r##"{"list":[{"content":"Task 1","priority":"High","status":false,"created":"26-05-2024"},{"content":"Task 2","priority":"High","status":false,"created":"23-06-2024"},{"content":"Task 3","priority":"High","status":false,"created":"22-01-2022"}]}"##.to_string();

    assert_eq!(content, should_be)
}

#[test]
fn back_up_file_todo() {
    //  Clean files before use
    let path = "tests/test_todo/.todo/save.todo".to_string();
    let path_backup = "tests/test_todo/.todo/backup.todo".to_string();

    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(&path).unwrap();

    let file_backup = OpenOptions::new()
    .read(true)
    .write(true)
    .open(&path_backup).unwrap();

    // Clearing the save file
    match file.set_len(0) {
        Ok(_) => (),
        Err(e) => println!("Error occured {e}"),
    };

    // Clearing the save file
    match file_backup.set_len(0) {
        Ok(_) => (),
        Err(e) => println!("Error occured {e}"),
    };

    //Creating an instance of the todo list
    let list_element = vec![
        TodoElement {content  : "Task 1".to_string(), priority :  Priority::High, status : false, created : "26-05-2024".to_string()},
        TodoElement {content  : "Task 2".to_string(), priority :  Priority::High, status : false, created : "23-06-2024".to_string()},
        TodoElement {content  : "Task 3".to_string(), priority :  Priority::High, status : false, created : "22-01-2022".to_string()},
    ];

    let todo_list = TodoList{
        list : list_element,
        path : path.clone(),
        path_backup : path_backup.clone(),
    };

    todo_list.write_file().expect("Didn't saved the file... ");
    todo_list.backup_data().expect("Didn't backup data...");

    //Check if the file contains the right json string
    let content = read_to_string(&path_backup).unwrap();
    let should_be = r##"{"list":[{"content":"Task 1","priority":"High","status":false,"created":"26-05-2024"},{"content":"Task 2","priority":"High","status":false,"created":"23-06-2024"},{"content":"Task 3","priority":"High","status":false,"created":"22-01-2022"}]}"##.to_string();

    assert_eq!(content, should_be)
}