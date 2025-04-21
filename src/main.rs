
use log;
use env_logger;
use std::path::Path;
use std::fs;
use std::process::Command;
use std::sync::Arc;
use std::env;
use std::path::{PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;
use chrono::{Datelike, Local, Timelike};
use directories::ProjectDirs;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use tokio::task;
use crate::task_definition::TaskDefinition;

pub mod utils;
pub mod task_definition;

use utils::{check_cron_field};


fn load_task_definitions(task_dir: &PathBuf) -> Vec<TaskDefinition>{
    let task_definition_paths = utils::load_tasks_from_fs(task_dir);
    let mut task_definitions: Vec<task_definition::TaskDefinition> = vec![];

    for path in task_definition_paths{
        let contents = fs::read_to_string(task_dir.join(&path));
        match contents{
            Ok(json) => {
                match TaskDefinition::from_json(&json){
                    Ok(td) => task_definitions.push(td),
                    Err(error) => log::warn!("Failed to parse {:?}: {:?}", path.to_string(), error)
                }
            },
            Err(error) => log::warn!("Failed to read {:?}: {:?}", path.to_string(), error)
        }

    }
    task_definitions
}

// compute the task dir
fn get_tasks_dir(proj_dirs: ProjectDirs) -> PathBuf{
    if let Ok(value) = env::var("PEND_TASK_DIR"){
        return Path::new(&value).to_path_buf();
    }
    let tasks_dir = proj_dirs.data_dir().join("tasks");
    fs::create_dir_all(&tasks_dir).expect("Failed to create tasks dir");
    tasks_dir
}

// multi_thread to enable parallelism
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // initialize logging
    env_logger::init();

    let proj_dirs = ProjectDirs::from("com", "example",
                                      "pend").expect("Failed to open config dirs");

    let tasks_dir = get_tasks_dir(proj_dirs);

    log::info!("Started tasks scheduler daemon.");

    // copy task_dir so the watcher can take ownership
    let watch_dir = tasks_dir.clone();

    // flags used by the watcher to mark when tasks are added/deleted
    let reload_flag = Arc::new(AtomicBool::new(false));

    let reload_flag_main = Arc::clone(&reload_flag);
    let reload_flag_watcher = Arc::clone(&reload_flag);

    let mut task_definitions: Vec<TaskDefinition> = load_task_definitions(&tasks_dir);
    let (tx, rx) = channel();

    let tasks_dir_clone = tasks_dir.clone();

    task::spawn(async move {
        log::info!("Watching task dir {:?}", watch_dir);

        // TODO: make this more resilient, i.e. program should still work with a broken watcher
        let mut watcher = recommended_watcher(tx)
            .expect("Failed to set up watcher");
        watcher.watch(Path::new(&tasks_dir_clone), RecursiveMode::Recursive).expect("Watcher failed");

        loop{
            match rx.recv(){
                Ok(_) => {
                    log::debug!("Change detected, marking reload...");
                    reload_flag_watcher.store(true, Ordering::Relaxed)
                },

                Err(_) => {} // TODO
            }
        }
    });

    // TODO: delay startup to align exactly with :00
    loop{
        // if the watcher indicates a reload is needed, refresh the task definitions
        if reload_flag_main.load(Ordering::Relaxed){
            log::info!("Reloading task definitions...");
            task_definitions = load_task_definitions(&tasks_dir);
        } else {
            log::debug!("No changes to task dir")
        }
        let now = Local::now();

        for td in &task_definitions{
            let cron_fields: Vec<&str>= td.cron_expr.split(" ").collect();
            match cron_fields.as_slice(){
                // TODO: handle cases where a cron_expr is valid followed by unnecessary characters
                [m, h, d, mo, w] => {
                    let m_match = check_cron_field(m, |x| x == now.minute());
                    let h_match = check_cron_field(h, |x| x == now.hour());
                    let d_match = check_cron_field(d, |x| x == now.day());
                    let mo_match = check_cron_field(mo, |x| x == now.month());
                    let w_match = check_cron_field(w, |x| x == now.weekday()
                        .num_days_from_sunday());

                    if m_match && h_match && d_match && mo_match && w_match{
                        log::info!("Firing off task...");
                        td.launch();
                    }
                },
                _ => {
                    log::warn!("Malformed cron expression, cannot process")
                }
            }
        }

        // sleep for 60 seconds (set to 10 right now to debug more easily)
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}