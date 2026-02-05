/**
 * @file main.rs
 * @author Yefri Encarnación (https://yeffej.github.io/)
 * @brief Main entry point of the TODO CLI application
 * @version 0.1
 * @date 2025-12-16
 *
 * @dev-thinking The idea is to receive the first argument (commands) which will determine the action
 * that the program will execute such as add (add / save a task in the db),
 * list (list all the task in the db showing its status and title), mark (change the status
 * of a selected task, in this case a menu has to be shown to select the task), remove (delete from
 * database the selected task, in this case a menu has to be shown to select the task)
 */
mod todo;

use std::env;
use std::io;
use std::process;
use todo::{Status, Todo};

enum Commands {
    Add,
    List,
    Mark,
    Remove,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command");
        process::exit(1);
    }

    let command = match args[1].as_str() {
        "add" => Commands::Add,
        "list" => Commands::List,
        "done" => Commands::Mark,
        "remove" => Commands::Remove,
        _ => {
            println!("Invalid command");
            process::exit(1);
        }
    };

    match command {
        Commands::Add => {
            if args.len() < 3 {
                println!("Please provide a title for the task");
                process::exit(1);
            }

            let title = args[2].clone();
            // create a new todo and save it to database
            let task = Todo::new(title, Status::ToDo);
            task.save();
        }
        Commands::List => todo::list(false), // list all todos from database
        Commands::Mark => {
            // show a menu to select the todo to mark as done / progress / todo
            println!("Select the TODO that you wish to change status.");
            todo::list(true);
            print!("Your selection: ");
            let mut todo_id = String::new();
            io::stdin().read_line(&mut todo_id).unwrap_or_default();

            let todo_id: i32 = todo_id.parse().unwrap_or_else(|err| {
                println!("Error parsing user input: {}", err);
                return 0;
            });

            // ask for the new status
            println!("You have selected: ");
            print!("New status: ");
            let mut new_status = String::new();
            io::stdin().read_line(&mut new_status).unwrap_or_default();

            let new_status = match new_status.as_str() {
                "todo" => Some(Status::ToDo),
                "progress" => Some(Status::Progress),
                "Done" => Some(Status::Done),
                _ => None,
            };

            // update the status of the selected todo
            let selected_todo = todo::get(todo_id);

            // save the updated todo in database
            // todo::mark()
        }
        Commands::Remove => {
            // todo::remove()
        }
    }

    print!("hello, world!");
}
