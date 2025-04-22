pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use std::{env, fs};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use directories::{ProjectDirs};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDefinition{
    //pub name: String,
    pub exec: String,
    pub args: Vec<String>,
    pub cron_expr: String
}


impl TaskDefinition{
    pub fn new(exec: String, args: Vec<String>, cron_expr: String, name: String) -> TaskDefinition {
        TaskDefinition{
            exec,
            args,
            cron_expr,
            //name
        }
    }

    pub fn from_json(json_str: &str) -> Result<TaskDefinition, serde_json::Error>{
        serde_json::from_str::<TaskDefinition>(&json_str)
    }
}



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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
