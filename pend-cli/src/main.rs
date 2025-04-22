use clap::{Parser, Subcommand};
use pend_core::{TaskDefinition};
use std::fs::OpenOptions;
use std::io::Write;
use ulid::Ulid;

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
}

#[derive(Parser, Debug)]
struct CreateTaskArgs {
    cron_expr: String,
    exec: String,
    #[arg(last = true)]
    exec_args: Vec<String>,
}

fn create_task(args: CreateTaskArgs) {
    let td = TaskDefinition::new(args.exec, args.exec_args, args.cron_expr);
    let json_str : String;


    // TODO: input validation for cron_expr

    match serde_json::to_string(&td){
        Ok(json) => {
            json_str = json;
        },
        _ => panic!("Failed to serialize task definition")
    }

    // TODO: load tasks_dir from envvar or config
    let tasks_dir = "./dev_configs";
    let filename = format!("{}/{}.json", tasks_dir, Ulid::new());

    let mut file = OpenOptions::new().append(true).create(true).open(filename)
        .expect("Could not create new task definition file");

    file.write_all(json_str.as_bytes()).expect("Failed to create task");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateTask(args) => create_task(args),
    }
}
