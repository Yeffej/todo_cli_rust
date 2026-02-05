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
use services::todo as db;

#[derive(Debug)]
pub enum Status {
    ToDo,
    Progress,
    Done,
}

pub struct Todo {
    title: String,
    status: Status,
    // created_at: String,
}

impl Todo {
    pub fn new(title: String, status: Status) -> Self {
        Todo { title, status }
    }

    pub fn set_title(&mut self, value: String) {
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

    pub fn save(&self) {
        db::save(self);
    }
}

pub fn list(as_menu: bool) {
    let todos = db::get_all();
    let mut count = 0;

    println!("ToDo List:");
    println!();
    for todo in &todos {
        count += 1;
        if as_menu {
            todo.display(Some(count));
        } else {
            todo.display(None);
        }
    }
}

pub fn get(id: i32) -> Option<Todo> {
    db::get_by_id(id)
}

pub fn mark() {}
