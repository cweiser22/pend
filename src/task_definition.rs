use std::process::Command;
use serde::{Serialize, Deserialize};
use tokio::task;
use crate::task_definition;


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDefinition{
    pub exec: String,
    pub args: Vec<String>,
    pub cron_expr: String
}

impl TaskDefinition{
    pub fn launch(&self){
        let exec = self.exec.clone();
        let args = self.args.clone();
        task::spawn(async {
            let output = Command::new(exec).args(args).output();

            match output{
                Ok(out) => log::debug!("stdout {:?}, stderr {:?}",String::from_utf8(out.stdout),
                    String::from_utf8(out.stderr)),
                Err(error) => log::debug!("{:?}", error)
            }
        });
    }

    pub fn new(exec: String, args: Vec<String>, cron_expr: String) -> TaskDefinition {
        TaskDefinition{
            exec,
            args,
            cron_expr
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