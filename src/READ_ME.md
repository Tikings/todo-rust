arguments for the Todo CLI : 

# Arguments that will be used in common cases : 

- Add -> "-a" , "--add" : To add elements to the todo list
    Subcommands : 
        - Priority -> "-p", "--priority" : Add the priority to an element of the todolist, if not mentionned for a task => set to "low" priority

- Done -> "-d", "--done" : Set the task status to done. 
    Subcommand : 
    - "-r" : remove the tasks that are set as done. 

- Restore -> "-u", "--undo" : Undo changes and get back the last version of the todolist (copy the content of the backup file to the main file)
- Sort "-s", "--sort": Display the task sorted by priority
- Reset -> "-c" , "--clear" : Reset the todo list == Clear the save file.

# Command lines that I wanted to implement but not sure if it is useful 
- Remove -> "delete" : remove the content by ID
    Subocommand :
    "--done" : Remove the task that are done

# Future implementations : 
- Add autocompletion for remove command : allows to remove a task by its content and not by its ID. 

# Can be implemented in the future for further use : 
- Raw : To have the data formated as a json (as it is saved in the save file). 

# How it is supposed to look like : 

The list display => The default representation 
Displayed with colors : Different colors for each priority 

```
TODO : {parent directory of the todo list}
    1. Task 4 
    2. Task 2 
    3. Task 3

DONE : 
    4. Task 1
```

With the sort command : 
```
TODO : {Parent directory}
- - HIGH PRIORITY - - 
    1.
    2.
    3.

- - MEDIUM PRIORITY - - 
    4.
    5.
    6.

- - LOW PRIORITY - - 
    7.
    8.
    9.

```
=> In this format the task that are done will not be displayed. 
