use std::fs::OpenOptions;
use clap::Parser;
use pend_core::get_tasks_dir;
use std::io::Write;
use std::process::exit;
use crate::editor::open_editor;

const TASK_BOILERPLATE: &str = r#"{
    "exec": "echo",
    "args": ["hello"],
    "cron_expr":"* * * * *"
}"#;

#[derive(Parser, Debug)]
pub struct CreateTaskArgs {
    /// Name of the task
    #[arg(short = 'n', long = "name")]
    pub name: String,
    /// Open the task for editing after creation
    #[arg(long = "edit")]
    edit: bool,
}

pub fn create_task_command(args: CreateTaskArgs) {
    let tasks_dir = get_tasks_dir();
    let filename = tasks_dir.join(format!("{}.json", args.name));

    // create new task file
    let mut file = OpenOptions::new().write(true).create_new(true).create(true).open(&filename)
        .unwrap_or_else(|e| {
        eprintln!("Could not create file: {}", e);
        exit(1);
    });

    // write the json boilerplate
    file.write_all(TASK_BOILERPLATE.as_bytes()).expect("Failed to create task");

    // immediately launch interactive editor if --edit
    if args.edit{
        open_editor(filename);
    }
}
