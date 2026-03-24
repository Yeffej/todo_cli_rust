use super::database::{self, TODOS_PATH};
use crate::todo::Todo;
use anyhow::Context;
use serde_json;
use std::fs;
use std::path::Path;

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
pub fn get_by_id(id: &String) -> anyhow::Result<Todo> {
    database::init()?;
    let todo_file_path = Path::new(TODOS_PATH).join(format!("{}.json", id));
    // read the file and parse it to Todo struct
    let content = fs::read_to_string(&todo_file_path).context("Failed to read todo file")?;
    let todo: Todo = serde_json::from_str(&content).context("Failed to parse todo")?;
    Ok(todo)
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
pub fn delete(id: &String) -> anyhow::Result<()> {
    database::init()?;
    let path = Path::new(TODOS_PATH).join(format!("{}.json", id));
    fs::remove_file(path).context("Failed to delete todo")?;
    Ok(())
}
