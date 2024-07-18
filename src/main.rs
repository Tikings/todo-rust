use todo::todo_list::TodoList;
use todo::todo_element::{TodoElement,Priority};

fn main() {

    let path = "tests/test_todo/.todo/save.todo".to_string();
    let path_backup = "tests/test_todo/.todo/backup.todo".to_string();

    let list_element = vec![
        TodoElement {content  : "Task 1".to_string(), priority :  Priority::Medium, status : false, created : "26-05-2024".to_string(), hash : "CdfxrIRnWyAZlBczamzysFLOPcod07al".to_string()},
        TodoElement {content  : "Task 2".to_string(), priority :  Priority::Low, status : true, created : "23-06-2024".to_string(), hash : "ASD0v9lXsMo0pacqL7BkfRCMWpI6a1pd".to_string()},
        TodoElement {content  : "Task 4".to_string(), priority :  Priority::Low, status : false, created : "22-01-2022".to_string(), hash : "yPrTtABuiQTDxRvTBHXHYI0MypGJsGen".to_string()},
        TodoElement {content  : "Task 5".to_string(), priority :  Priority::Medium, status : false, created : "31-07-2025".to_string(), hash : "tWaO17BhhRfm7MYy0iS67AAtNwLSoBql".to_string()},
        TodoElement {content  : "Task 6".to_string(), priority :  Priority::High, status : true, created : "04-01-2002".to_string(), hash : "cn3B3VzUysOfZdVS9q6Jt3s1YKIeQ602".to_string()},
        TodoElement {content  : "Task 7".to_string(), priority :  Priority::High, status : false, created : "05-04-1999".to_string(), hash : "mQJTBMcvQoPcBVAJ4mnyj4Np07LrDBsk".to_string()},
    ];

    let mut todo_list = TodoList{
        list : list_element,
        path : path.clone(),
        path_backup : path_backup.clone(),
        hash_list : vec![ "CdfxrIRnWyAZlBczamzysFLOPcod07al".to_string(), "ASD0v9lXsMo0pacqL7BkfRCMWpI6a1pd".to_string(), "yPrTtABuiQTDxRvTBHXHYI0MypGJsGen".to_string()],
    };
    
    todo_list.write_file().unwrap();

    println!("{:?}", todo_list.hash_list);

    todo_list.display_by_priority().unwrap();  

    todo_list.write_file().unwrap();

    println!("{:?}", todo_list.hash_list);
}

