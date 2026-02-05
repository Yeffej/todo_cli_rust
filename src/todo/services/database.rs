use std::fs;
use std::path::Path;

struct Database {
    base: String,
    todos_path: String,
}

impl Database {
    fn new() -> anyhow::Result<Self> {
        let path = Path::new("/app_data/todos");
        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        Ok(Database {
            base: String::from("/app_data"),
            todos_path: String::from("/app_data/todos"),
        })
    }
}

// use std::sync::OnceLock;

// struct Database {
//     base: String,
//     todos_path: String,
// }

// impl Database {
//     fn new() -> anyhow::Result<Self> {
//         let path = Path::new("/app_data/todos");
//         if !path.exists() {
//             fs::create_dir_all(path)?;
//         }

//         Ok(Database {
//             base: String::from("/app_data"),
//             todos_path: String::from("/app_data/todos"),
//         })
//     }

//     pub fn get_instance() -> anyhow::Result<&'static Database> {
//         static INSTANCE: OnceLock<Database> = OnceLock::new();
//         Ok(INSTANCE.get_or_init(|| Database::new()?))
//     }
// }
