use anyhow::Context;
use std::{env, fs};

pub const TODOS_PATH: &str = "app_data/todos";

pub fn init() -> anyhow::Result<()> {
    let base_path = env::current_dir().context("Failed to get current directory")?;
    let path = base_path.join(TODOS_PATH);
    if !path.exists() {
        fs::create_dir_all(&path).context("Database initialization failed")?;
    }

    Ok(())
}
