use std::process::Command;
use serde::{Serialize, Deserialize};
use tokio::task;
use crate::task_definition;

fn complete_default() -> bool{
    true
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDefinition{
    pub exec: String,
    pub args: Vec<String>,
    #[serde(default="complete_default")]
    pub complete: bool,
    pub cron_expr: String
}

impl TaskDefinition{
    pub fn launch(&self){
        let exec = self.exec.clone();
        let args = self.args.clone();
        task::spawn(async {
            let output = Command::new(exec).args(args).output()
                .expect("Failed to run tasks");
            //log::info!("{:?}", output);
        });
    }

    pub fn new(exec: String, args: Vec<String>, cron_expr: String) -> TaskDefinition {
        TaskDefinition{
            exec,
            args,
            cron_expr,
            complete: false
        }
    }

    pub fn from_json(json_str: &str) -> Result<TaskDefinition, serde_json::Error>{
        serde_json::from_str::<TaskDefinition>(&json_str)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_definition_with_json(){
        let json = r#"
        {
            "exec": "echo",
            "args": ["hi"],
            "complete": false,
            "cron_expr": "* * * * *"
        }
        "#;

        let _: TaskDefinition = serde_json::from_str(json).unwrap();
    }

}