mod create;
mod edit;
mod editor;
mod delete;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pend-cli")]
#[command(about = "A multi-binary CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create(create::CreateTaskArgs),
    Edit(edit::EditTaskArgs),
    Delete(delete::DeleteTaskArgs),
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create(args) => create::create_task_command(args),
        Commands::Edit(args) => edit::edit_task_command(args),
        Commands::Delete(args) => delete::delete_task_command(args)
    }
}
