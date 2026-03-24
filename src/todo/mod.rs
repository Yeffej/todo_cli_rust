/**
 * @file todo.rs
 * @author Yefri Encarnación (https://yeffej.github.io/)
 * @brief Module of Todo where all services or actions related to task/todo will be available.
 * @version 0.1
 * @date 2025-12-16
 */
// use crate::services::todo as db;
// use services::todo as db;
mod services;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use services::todo as db;
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    ToDo,
    Progress,
    Done,
}

impl Status {
    pub const ALL_STATUSES: [Status; 3] = [Status::ToDo, Status::Progress, Status::Done];
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: String,
    title: String,
    status: Status,
    created_at: String,
}

impl Todo {
    pub fn create(title: String, status: Status) -> Self {
        Todo {
            id: Uuid::new_v4().simple().to_string(),
            title,
            status,
            created_at: String::from("2025-12-16"),
        }
    }
    pub fn _set_title(&mut self, value: String) {
        self.title = value
    }

    pub fn set_status(&mut self, value: Status) {
        self.status = value
    }

    pub fn display(&self, num_order: Option<i32>) {
        match num_order {
            Some(n) => println!("{}- [{:?}]: {}", n, self.status, self.title),
            None => println!("- [{:?}]: {}", self.status, self.title),
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        db::save(self)?;
        Ok(())
    }
}

pub fn list(as_menu: bool) -> anyhow::Result<Option<String>> {
    let todos = db::get_all()?;

    if as_menu {
        let mut count = 0;
        // show menu to select a todo
        println!("Select the ToDo that you wish to change status:");
        for todo in &todos {
            count += 1;
            todo.display(Some(count));
        }

        let mut selected = String::new();
        print!("ToDo (Type the number): ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut selected).unwrap_or_default();

        let idx = selected
            .trim()
            .parse::<usize>()
            .context("incorrect todo selection. The selection should be a integer")?;

        match todos.get(idx - 1) {
            Some(todo) => {
                println!("You have selected: {}", todo.title);
                return Ok(Some(todo.id.clone()));
            }
            None => anyhow::bail!(
                "Invalid selection. Please select a number between 1 and {}",
                todos.len()
            ),
        };
    } else {
        println!("ToDo List:");
        todos.iter().for_each(|todo| todo.display(None));
        Ok(None)
    }
}

pub fn get(id: &String) -> anyhow::Result<Todo> {
    db::get_by_id(id)
}

pub fn mark(id: &String, status: Status) -> anyhow::Result<()> {
    let mut todo = get(id)?;
    todo.set_status(status);
    todo.save()?;
    Ok(())
}

pub fn remove(id: &String) -> anyhow::Result<()> {
    db::delete(id)
}
