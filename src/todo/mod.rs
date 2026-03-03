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
use serde::{Deserialize, Serialize};
use services::todo as db;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    Progress,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: String,
    title: String,
    status: Status,
    created_at: String,
}

impl Todo {
    fn new(id: String, title: String, status: Status, created_at: String) -> Self {
        Todo {
            id,
            title,
            status,
            created_at,
        }
    }

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

    pub fn _set_status(&mut self, value: Status) {
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

pub fn list(as_menu: bool) -> anyhow::Result<()> {
    let todos = db::get_all()?;
    let mut count = 0;

    println!("ToDo List:");
    for todo in &todos {
        count += 1;
        if as_menu {
            todo.display(Some(count));
        } else {
            todo.display(None);
        }
    }

    Ok(())
}

pub fn get(id: String) -> Option<Todo> {
    let id = Uuid::parse_str(&id);
    if let Ok(id) = id {
        return db::get_by_id(id);
    }
    None
}

pub fn _mark() {}
