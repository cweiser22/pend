use clap::Parser;
use pend_core::get_tasks_dir;
use crate::editor;

#[derive(Parser, Debug)]
pub struct EditTaskArgs {
    /// Name of the task
    #[arg(short = 'n', long = "name")]
    pub name: String,
    /// Open the task for editing after creation
    #[arg(long = "edit")]
    edit: bool,
}

pub fn edit_task_command(args: EditTaskArgs){
    let tasks_dir = get_tasks_dir();
    let filename = tasks_dir.join(format!("{}.json", args.name));
    if (!filename.exists()){
        panic!("Cannot edit {} as it does not exist.", args.name);
    }
    editor::open_editor(filename);
}
