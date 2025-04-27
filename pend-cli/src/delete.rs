use std::fs;
use std::process::exit;
use clap::Parser;
use pend_core::get_tasks_dir;
use crate::editor;

#[derive(Parser, Debug)]
pub struct DeleteTaskArgs {
    /// Name of the task to be deleted
    #[arg(short = 'n', long = "name")]
    pub name: String,
}

pub fn delete_task_command(args: DeleteTaskArgs){
    let tasks_dir = get_tasks_dir();
    let filename = tasks_dir.join(format!("{}.json", args.name));
    if !filename.exists(){
        eprintln!("Cannot delete {} as it does not exist.", args.name);
        exit(1);
    } else {
        fs::remove_file(filename).unwrap_or_else(|e| {
            eprintln!("Failed to delete task: {}", e);
            exit(1);
        });
    }
}
