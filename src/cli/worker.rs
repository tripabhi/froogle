use crate::model::cache;
use crate::{model::Model, parser, util};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

pub fn start_indexing(dir: &Path, model: Arc<Mutex<Model>>) {
    let path = dir.to_path_buf();
    thread::spawn(move || {
        if let Ok(processed) = index_dir(&path, Arc::clone(&model)) {
            log::info!("Finished indexing directory");
            if processed > 0 {
                let model = model.lock().unwrap();
                let cache_file_path = path.join(cache::CACHE_FILE_NAME);
                if model.save_to_file(&cache_file_path).is_ok() {
                    log::info!("Saved model to file");
                }
            }
        };
    });
}

pub fn index_dir(dir: &Path, model: Arc<Mutex<Model>>) -> Result<usize, ()> {
    let mut processed: usize = 0;
    let dir_path = dir.to_path_buf();
    let dir = fs::read_dir(dir).map_err(|err| {
        log::error!("Cannot open directory {dir_path:?} : {err}");
    })?;

    'next: for file in dir {
        if let Ok(file_entry) = file {
            let file_path = file_entry.path();

            // Ignore dot files
            if util::is_dot_file(&file_path) {
                continue 'next;
            }

            let is_dir = match util::is_dir(&file_entry) {
                Ok(is_dir) => is_dir,
                Err(_) => continue 'next,
            };

            // Recursively index directory
            if is_dir {
                if let Ok(total) = index_dir(&file_path, Arc::clone(&model)) {
                    processed += total;
                }
                continue 'next;
            }

            match util::get_last_modified(&file_entry) {
                Ok(last_modified) => {
                    let mut model = model.lock().unwrap();
                    if model.should_index(&file_path, last_modified) {
                        model.remove_document(&file_path);

                        log::debug!("Indexing... {file_path:?}");

                        let content = match parser::parse_file_by_extension(&file_path) {
                            Ok(s) => s.chars().collect::<Vec<_>>(),
                            Err(_) => continue 'next,
                        };

                        model.add_document(file_path, last_modified, &content);
                        processed += 1;
                    }
                }
                Err(_) => continue 'next,
            }
        } else {
            log::error!("Cannot read next file in {dir_path:?}");
            continue 'next;
        }
    }
    Ok(processed)
}
