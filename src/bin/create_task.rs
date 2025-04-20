use std::fs::OpenOptions;
use std::io::Write;
use clap::Parser;
use pend::task_definition;
use ulid::Ulid;

#[derive(Parser, Debug)]
struct Args{
    cron_expr: String,
    exec: String,
    #[clap(last=true)]
    exec_args: Vec<String>
}

fn main() {
    let args = Args::parse();
    let td = task_definition::TaskDefinition::new(args.exec, args.exec_args, args.cron_expr);
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