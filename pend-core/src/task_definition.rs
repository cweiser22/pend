use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDefinition{
    //pub name: String,
    pub exec: String,
    pub args: Vec<String>,
    pub cron_expr: String
}


impl TaskDefinition {
    pub fn new(exec: String, args: Vec<String>, cron_expr: String) -> TaskDefinition {
        TaskDefinition {
            exec,
            args,
            cron_expr,
        }
    }

    pub fn from_json(json_str: &str) -> Result<TaskDefinition, serde_json::Error>{
        serde_json::from_str::<TaskDefinition>(&json_str)
    }
}