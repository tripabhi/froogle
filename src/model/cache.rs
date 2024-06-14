use super::Model;
use std::{fs::File, io::BufReader, path::Path};

pub const CACHE_FILE_NAME: &str = ".frcache.json";

pub fn get_model(search_path: &Path) -> Option<Model> {
    let cache_file_path = search_path.to_path_buf().join(CACHE_FILE_NAME);

    let cache_exists = cache_file_path
        .try_exists()
        .map_err(|err| {
            log::error!(
                "Cannot check existence of cache file {file:?} : {err}",
                file = &cache_file_path
            );
        })
        .unwrap_or_default();

    if cache_exists {
        match File::open(&cache_file_path) {
            Ok(cache_file) => {
                let model = serde_json::from_reader(BufReader::new(cache_file));
                match model {
                    Ok(model) => return Some(model),
                    Err(err) => {
                        log::error!(
                            "Cannot deserialize {file:?} to model : {err}",
                            file = &cache_file_path
                        );
                    }
                };
            }
            Err(err) => {
                log::error!(
                    "Cannot open cache file {file:?} : {err}",
                    file = &cache_file_path
                );
            }
        }
    }
    None
}
