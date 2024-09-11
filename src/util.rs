use path_dedot::ParseDot;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// This utility performs path expansion based on dot, tilde 
/// and shell based expansions.
pub fn expand_path(path: impl AsRef<str>) -> Result<PathBuf, ()> {
    let tilde_and_env_expanded_path = shellexpand::full(path.as_ref()).map_err(|err| {
        log::error!(
            "Error while expanding {path:?} : {err}",
            path = path.as_ref()
        );
    })?;

    let result = Path::new(tilde_and_env_expanded_path.as_ref())
        .parse_dot()
        .map_err(|err| {
            log::error!(
                "Error while de-doting {path:?} : {err}",
                path = tilde_and_env_expanded_path.as_ref()
            );
        })?;

    Ok(result.to_path_buf())
}

pub fn get_last_modified(file_entry: &DirEntry) -> Result<SystemTime, ()> {
    let path = file_entry.path();
    return file_entry
        .metadata()
        .map_err(|err| {
            log::error!("Cannot read metadata of file {path:?} : {err}");
        })
        .ok()
        .map(|metadata| {
            metadata.modified().map_err(|err| {
                log::error!("Cannot read last modified time of {path:?} : {err}");
            })
        })
        .unwrap_or(Err(()));
}

pub fn is_dot_file(file_path: &PathBuf) -> bool {
    file_path
        .file_name()
        .and_then(|f| f.to_str())
        .map(|s| s.starts_with("."))
        .unwrap_or_default()
}

pub fn file_extension(file_path: impl AsRef<Path>) -> Option<String> {
    file_path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_string())
}

pub fn is_dir(entry: &DirEntry) -> Result<bool, ()> {
    match entry.file_type() {
        Ok(file_type) => Ok(file_type.is_dir()),
        Err(err) => {
            log::error!(
                "Cannot detect file type of {file:?} : {err}",
                file = entry.file_name()
            );
            Err(())
        }
    }
}
