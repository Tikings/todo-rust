use clap::{ value_parser, Arg, ArgAction, Command}; 

pub fn cli() -> Command {
    Command::new("todo")
    .about("A to-do list to keep track")
    .subcommand_required(true)
    .subcommand(
        Command::new("add")
            .about("To add a task to the todo list")
            .arg_required_else_help(true)
            .arg(
                Arg::new("task")
                .required(false)
                .short('t')
                .long("task")
                .action(ArgAction::Set)
                .value_name("TASK")
                .value_parser(value_parser!(String))
                .help("The task to add to the to-do list")
            )
            .arg(
                Arg::new("priority")
                .required(false)
                .short('p')
                .long("priority")
                .action(ArgAction::Set)
                .value_name("PRIO")
                .value_parser(value_parser!(String))
                .default_value("m")
                .help("Set the priority of the task.")
            )
    )
    .subcommand(
        Command::new("done")
            .about("To set a task as done in the to-do list")
            .arg_required_else_help(true)
            .arg(
                Arg::new("id")
                .required(false)
                .long("id")
                .action(ArgAction::Set)
                .value_name("ID")
                .value_parser(value_parser!(usize))
                .help("Id of the task to set as done.")
            )
        )
    .subcommand(
        Command::new("remove")
            .about("To remove a task from the todo list")
            .arg_required_else_help(true)
            .alias("delete")
            .arg(
                Arg::new("id")
                .required(false)
                .long("id")
                .action(ArgAction::Set)
                .value_name("ID")
                .value_parser(value_parser!(usize))
                .help("Id of the task to remove")
            )
        )
    .subcommand(
        Command::new("restore")
        .alias("undo")
        .about("To undo the last changes of the to-do list.")
    )
    .subcommand(
        Command::new("reset")
        .alias("clear")
        .about("Clear the todo list.")
    )
    .subcommand(
        Command::new("sort")
        .alias("by-priority")
        .about("Display the todo list sorted by priority.")
    )
    .subcommand(
        Command::new("list")
        .alias("display")
        .about("Display the to-do list sorted by dates.")
    )

}