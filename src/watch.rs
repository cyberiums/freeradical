extern crate notify;

use actix_web::web::Data;
use handlebars::DirectorySourceOptions;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::time::Duration;

/// Watches the templates directory and refreshes the templates in memory on update.
pub fn watch(hb: Data<Mutex<handlebars::Handlebars<'_>>>) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch("./templates", RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(_) => {
                hb.lock().unwrap().clear_templates();
                let mut options = DirectorySourceOptions::default();
                options.tpl_extension = ".hbs".to_string();
                hb.lock()
                    .unwrap()
                    .register_templates_directory("./templates", options)
                    .unwrap();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
