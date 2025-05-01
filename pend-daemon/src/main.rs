
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
use tokio::task;
use pend_core::{TaskDefinition, get_tasks_dir};

mod utils;
mod watcher;

use utils::{check_cron_field, load_task_definitions};

fn launch_task(td: &TaskDefinition){
    let exec = td.exec.clone();
    let args = td.args.clone();
    task::spawn(async {
        let output = Command::new(exec).args(args).output();

        match output{
            Ok(out) => log::debug!("stdout {:?}, stderr {:?}",String::from_utf8(out.stdout),
                    String::from_utf8(out.stderr)),
            Err(error) => log::debug!("{:?}", error)
        }
    });
}

// multi_thread to enable parallelism
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // initialize logging
    env_logger::init();

    let tasks_dir = get_tasks_dir();

    log::info!("Started tasks scheduler daemon.");

    // copy task_dir so the watcher can take ownership
    let watch_dir = tasks_dir.clone();

    // flags used by the watcher to mark when tasks are added/deleted
    let reload_flag = Arc::new(AtomicBool::new(false));

    let reload_flag_main = Arc::clone(&reload_flag);
    let reload_flag_watcher = Arc::clone(&reload_flag);

    let mut task_definitions: Vec<TaskDefinition> = load_task_definitions(&tasks_dir);
    

    let tasks_dir_clone = tasks_dir.clone();

    task::spawn(async move {
        loop {
            watcher::start_watcher(&tasks_dir_clone, &reload_flag_watcher);
            log::warn!("Watcher stopped running. Restarting...")
        }
    });

    // offset so that we run very close to :00
    let offset = utils::calculate_time_offset();
    log::info!("Launching in {} seconds...", offset);
    tokio::time::sleep(Duration::from_secs(utils::calculate_time_offset())).await;

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
                        launch_task(&td);
                    }
                },
                _ => {
                    log::warn!("Malformed cron expression, cannot process")
                }
            }
        }

        // sleep for 60 seconds (set to 10 right now to debug more easily)
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}