use crate::todo::{Status, Todo};

// get all todos from database
pub fn get_all() -> Vec<Todo> {
    vec![]
}

// get all todos from database that pass the filter
pub fn get_by_id(id: i32) -> Option<Todo> {
    Some(Todo::new(String::from("Testing"), Status::ToDo))
}

// Save todo to database
pub fn save(todo: &Todo) -> bool {
    false
}

// Delete todos from database that pass the filter
pub fn delete() -> bool {
    false
}
