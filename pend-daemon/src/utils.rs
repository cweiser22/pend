use std::{fs};
use std::path::{Path, PathBuf};
use pend_core::TaskDefinition;
use chrono::{Local, Timelike};

// returns the amount of time between now and the beginning of the next minute
pub fn calculate_time_offset() -> u64 {
    let now = Local::now();
    let seconds = now.second();
    let nanos = now.nanosecond();

    let secs_until_next = 60 - seconds - if nanos > 0 { 1 } else { 0 };
    secs_until_next as u64
}

// TODO: consider merging this into load_task_definition
fn load_tasks_from_fs(task_dir: &PathBuf) -> Vec<String>{
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

pub fn load_task_definitions(task_dir: &PathBuf) -> Vec<TaskDefinition>{
    let task_definition_paths = load_tasks_from_fs(task_dir);
    let mut task_definitions: Vec<TaskDefinition> = vec![];

    for path in task_definition_paths{
        let contents = fs::read_to_string(task_dir.join(&path));
        match contents{
            Ok(json) => {
                match TaskDefinition::from_json(&json){
                    Ok(td) => task_definitions.push(td),
                    Err(error) => log::warn!("Failed to parse {:?}: {:?}", path.to_string(), error)
                }
            },
            Err(error) => log::warn!("Failed to read {:?}: {:?}", path.to_string(), error)
        }
    }
    task_definitions
}


pub fn check_cron_field<F>(x: &str, f: F) -> bool where F: Fn(u32) -> bool{
    if x == "*"{
        true
    } else {
        match x.parse::<u32>(){
            Ok(n) => f(n),
            Err(e) => {
                log::warn!("Failed to parse u32 in cron_expr: {:?}", e);
                false
            }
        }
    }
}
