use todo::todo_list::TodoList;
use todo::todo_element::{TodoElement,Priority};

fn main() {

    let path = "tests/test_todo/.todo/save.todo".to_string();
    let path_backup = "tests/test_todo/.todo/backup.todo".to_string();

    let list_element = vec![
        TodoElement {content  : "Task 1".to_string(), priority :  Priority::Medium, status : false, created : "26-05-2024".to_string()},
        TodoElement {content  : "Task 2".to_string(), priority :  Priority::Low, status : true, created : "23-06-2024".to_string()},
        TodoElement {content  : "Task 3".to_string(), priority :  Priority::High, status : false, created : "22-01-2022".to_string()},
    ];

    let mut todo_list = TodoList{
        list : list_element,
        path : path.clone(),
        path_backup : path_backup.clone(),
    };

    // todo_list.write_file().expect("Didn't saved the file... ");
    // todo_list.sort_by_date();
    // todo_list.sort_by_priority();

    todo_list.display_by_date().unwrap();  
    // let todolist = TodoList::from_data(path).unwrap(); 

    // println!("{}",todo_list);
}

