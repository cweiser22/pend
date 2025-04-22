use std::path::PathBuf;
use std::process::Command;

pub fn open_editor(task_path: PathBuf){
    // TODO: make this configurable
    let editor = "vi";

    let status = Command::new(editor)
        .arg(&task_path)
        .status()
        .expect("failed to launch editor");

    if !status.success() {
        panic!("Editor exited with error");
    }

}