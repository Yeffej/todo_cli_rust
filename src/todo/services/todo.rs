use super::database::{self, TODOS_PATH};
use crate::todo::{Status, Todo};
use anyhow::Context;
use serde_json;
use std::fs;
use std::path::Path;
use uuid::Uuid;

// get all todos from database
pub fn get_all() -> anyhow::Result<Vec<Todo>> {
    database::init()?;
    // read all files from TODOS_PATH and parse them to Todos vector
    let todos = fs::read_dir(TODOS_PATH)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(|path| {
            if path.is_file() {
                // read the file and parse it to Todo struct
                let content = fs::read_to_string(path).unwrap_or_default();
                let todo: Result<Todo, serde_json::Error> = serde_json::from_str(&content);

                if let Ok(todo) = todo {
                    return Some(todo);
                }
            }

            None
        });

    Ok(todos.collect())
}

// get all todos from database that pass the filter
pub fn get_by_id(id: Uuid) -> Option<Todo> {
    Some(Todo::new(
        id.simple().to_string(),
        String::from("Testing"),
        Status::ToDo,
        String::from("2025-12-16"),
    ))
}

// Save todo to database
pub fn save(todo: &Todo) -> anyhow::Result<()> {
    database::init()?;
    let content = serde_json::to_string(todo).context("failed to parse todo")?;
    let path = Path::new(TODOS_PATH).join(format!("{}.json", todo.id));
    fs::write(path, content).context("failed to save todo")?;
    Ok(())
}

// Delete todos from database that pass the filter
pub fn _delete(id: Uuid) -> anyhow::Result<()> {
    database::init()?;
    let path = Path::new(TODOS_PATH).join(format!("{}.json", id.simple().to_string()));
    fs::remove_file(path).context("Failed to delete todo")?;
    Ok(())
}
