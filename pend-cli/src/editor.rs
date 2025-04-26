use std::path::PathBuf;
use std::process::{Command, exit};

pub fn open_editor(task_path: PathBuf){
    let editor = "vi";

    let status = Command::new(editor)
        .arg(&task_path)
        .status();


    match status{
        Ok(status) => {
            if !status.success() {
                eprintln!("Editor exited with error code: {}", status);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Could not launch editor: {}", e);
            exit(1);
        }
    }
}