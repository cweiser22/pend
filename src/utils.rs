use std::fs;
use std::path::Path;
use std::process::Command;


pub(crate) fn load_tasks_from_fs(task_dir: &str) -> Vec<String>{
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