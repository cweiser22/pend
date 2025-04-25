use std::{env, fs};
use std::path::{Path, PathBuf};
use directories::ProjectDirs;

pub fn get_tasks_dir() -> PathBuf{
    let proj_dirs = ProjectDirs::from("com", "example",
                                      "pend-cli").expect("Failed to open config dirs");
    if let Ok(value) = env::var("PEND_TASKS_DIR"){
        return Path::new(&value).to_path_buf();
    }
    let tasks_dir = proj_dirs.data_dir().join("tasks");
    fs::create_dir_all(&tasks_dir).expect("Failed to create tasks dir");
    tasks_dir
}