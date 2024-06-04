use todo::{self, CreationError, Priority, TodoElement};
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