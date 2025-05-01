use notify::{RecursiveMode, recommended_watcher, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;

pub fn start_watcher(watch_dir: &PathBuf, reload_flag: &Arc<AtomicBool>){
    let (tx, rx) = channel();

    log::info!("Watching task dir {:?}", watch_dir);

    // TODO: make this more resilient, i.e. program should still work with a broken watcher
    let mut watcher = recommended_watcher(tx);
    if let Ok(mut watcher) = watcher{
        let _ = watcher
            .watch(Path::new(&watch_dir), RecursiveMode::Recursive);
        
        loop {
            match rx.recv() {
                Ok(_) => {
                    log::debug!("Change detected, marking reload...");
                    reload_flag.store(true, Ordering::Relaxed)
                }

                Err(_) => {}
            }
        }
    }
}
