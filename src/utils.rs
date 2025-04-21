use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;



pub fn load_tasks_from_fs(task_dir: &PathBuf) -> Vec<String>{
    let mut task_definition_paths: Vec<String> = vec![];
    let task_path = Path::new(task_dir);
    for entry in fs::read_dir(task_path).unwrap(){
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension(){
                if ext == "json"{
                    log::debug!("Found task definition file {}", path.display());
                    task_definition_paths.push(entry.file_name().into_string().unwrap());
                }
            }
        }
    }
    task_definition_paths
}

pub fn check_cron_field<F>(x: &str, f: F) -> bool where F: Fn(u32) -> bool{
    if x == "*"{
        true
    } else {
        match x.parse::<u32>(){
            Ok(n) => f(n),
            Err(e) => {
                log::warn!("Failed to parse u32 in cron_expr");
                false
            }
        }
    }
}