use std::path::PathBuf;
use std::process::{Command, exit};

pub fn open_editor(task_path: PathBuf){
    let editor = "vi";

    let status = Command::new(editor)
        .arg(&task_path)
        .status()
        .expect("failed to launch editor");

    if !status.success() {
        eprintln!("Editor exited with error code: {}", status);
        exit(1);
    }

}