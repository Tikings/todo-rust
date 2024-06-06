use todo::{TodoElement, TodoList, Priority};

fn main() {

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
    todo_list.backup_data().expect("Problem during backup");
}
