use todo::{self, CreationError, Priority, TodoElement, TodoList};
use chrono::prelude::*;

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

    // TODO : Clean the file before use

    let list_element = vec![
        TodoElement {content  : "Task 1".to_string(), priority :  Priority::High, status : false, created : "26-05-2024".to_string()},
        TodoElement {content  : "Task 2".to_string(), priority :  Priority::High, status : false, created : "23-06-2024".to_string()},
        TodoElement {content  : "Task 3".to_string(), priority :  Priority::High, status : false, created : "22-01-2022".to_string()},
    ];
    let todo_list = TodoList{
        list : list_element,
        path : "tests/test_todo/.todo/save.todo".to_string(),
        path_backup :  "tests/test_todo/.todo/backup.todo".to_string(),
    };

    todo_list.write_file().expect("Didn't saved the file... ");

    //TODO : Check if the file contains the right json string
}

#[test]
fn back_up_file_todo() {

    // TODO : Clean the file before use

    let list_element = vec![
        TodoElement {content  : "Task 1".to_string(), priority :  Priority::High, status : false, created : "26-05-2024".to_string()},
        TodoElement {content  : "Task 2".to_string(), priority :  Priority::High, status : false, created : "23-06-2024".to_string()},
        TodoElement {content  : "Task 3".to_string(), priority :  Priority::High, status : false, created : "22-01-2022".to_string()},
    ];
    let todo_list = TodoList{
        list : list_element,
        path : "tests/test_todo/.todo/save.todo".to_string(),
        path_backup :  "tests/test_todo/.todo/backup.todo".to_string(),
    };

    todo_list.write_file().expect("Didn't saved the file... ");

    //TODO : Check if the file contains the right json string
}