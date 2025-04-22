use clap::{Parser, Subcommand};
use pend_core::{get_tasks_dir, TaskDefinition};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "pend-cli")]
#[command(about = "A multi-binary CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CreateTask(CreateTaskArgs),
    EditTask(EditTaskArgs)
}

#[derive(Parser, Debug)]
pub struct CreateTaskArgs {
    /// Name of the task
    #[arg(short = 'n', long = "name")]
    pub name: String,
    /// Open the task for editing after creation
    #[arg(long = "edit")]
    edit: bool,
}

#[derive(Parser, Debug)]
pub struct EditTaskArgs {
    /// Name of the task
    #[arg(short = 'n', long = "name")]
    pub name: String,
    /// Open the task for editing after creation
    #[arg(long = "edit")]
    edit: bool,
}

const TASK_BOILERPLATE: &str = r#"
{
    "exec": "echo",
    "args": ["hello"],
    "cron_expr":"* * * * *"
}
"#;

fn open_editor(task_path: PathBuf){
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

fn create_task(args: CreateTaskArgs) {
    let tasks_dir = get_tasks_dir();
    let filename = tasks_dir.join(format!("{}.json", args.name));

    let mut file = OpenOptions::new().write(true).create_new(true).create(true).open(&filename)
        .expect("Could not create new task definition file");

    file.write_all(TASK_BOILERPLATE.as_bytes()).expect("Failed to create task");

    if args.edit{
        open_editor(filename);
    }
}

fn edit_task(args: EditTaskArgs){
    let tasks_dir = get_tasks_dir();
    let filename = tasks_dir.join(format!("{}.json", args.name));
    if (!filename.exists()){
        panic!("Cannot edit {} as it does not exist.", args.name);
    }
    open_editor(filename);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateTask(args) => create_task(args),
        Commands::EditTask(args) => edit_task(args)
    }
}
