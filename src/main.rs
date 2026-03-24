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

use anyhow::Context;
use std::env;
use std::io;
use std::io::Write;
use std::process;
use todo::{Status, Todo};

enum Commands {
    Add,
    List,
    Mark,
    Remove,
}

fn main() -> anyhow::Result<()> {
    // parse arguments to determine the command to execute
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command");
        process::exit(1);
    }

    let command = match args[1].as_str() {
        "add" => Commands::Add,
        "list" => Commands::List,
        "mark" => Commands::Mark,
        "remove" => Commands::Remove,
        _ => {
            println!("Invalid command");
            process::exit(1);
        }
    };

    // execute the command
    match command {
        Commands::Add => {
            if args.len() < 3 {
                println!("Please provide a title for the task");
                process::exit(1);
            }

            let title = args[2].clone();
            // create a new todo and save it to database
            let task = Todo::create(title, Status::ToDo);
            match task.save() {
                Ok(()) => println!("Task added successfully!"),
                Err(err) => println!("Error adding task: {}", err),
            }
        }
        Commands::List => {
            todo::list(false)?;
        }
        Commands::Mark => {
            // show menu to select a todo
            if let Some(id) = todo::list(true)? {
                // show menu to select the new status
                println!("Available status:");
                for (idx, status) in Status::ALL_STATUSES.iter().enumerate() {
                    println!("{}. {:?}", idx + 1, status);
                }

                print!("New status (Type the number): ");
                let mut status_idx = String::new();
                io::stdout().flush().unwrap_or_default(); // ensure the prompt is printed before reading input
                io::stdin().read_line(&mut status_idx).unwrap_or_default();

                let status_idx = status_idx
                    .trim()
                    .parse::<usize>()
                    .context("Invalid status selection")?;
                let new_status = Status::ALL_STATUSES
                    .get(status_idx - 1)
                    .context("Status selection out of range")?;
                println!("You have selected: {:?}", new_status);

                // update the status of the selected todo
                if let Ok(_) = todo::mark(&id, new_status.clone()) {
                    println!("Task status updated successfully!");
                } else {
                    println!("Error updating task status");
                }
            }
        }
        Commands::Remove => {
            if let Some(id) = todo::list(true)? {
                if let Ok(_) = todo::remove(&id) {
                    println!("Task removed successfully!");
                } else {
                    println!("Error removing task");
                }
            }
        }
    }

    Ok(())
}
