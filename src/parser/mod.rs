use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub mod html;

pub mod xml;

pub mod pdf;

const BUFFER_CAPACITY: usize = 1024;
const UNSUPPORTED_FILE_EXTS: [&str; 2] = ["so", "o"];

pub fn parse_file_by_extension(path: &Path) -> Result<String, ()> {
    let extension = path
        .extension()
        .ok_or_else(|| {
            log::debug!("Can't detect extension of {path:?}");
        })?
        .to_string_lossy();

    match extension.as_ref() {
        "html" => Ok(html::read_file_to_string(path)?),
        "xml" | "xhtml" => Ok(xml::read_file_to_string(path)?),
        "pdf" => Ok(pdf::read_file_to_string(path)?),
        other => {
            if UNSUPPORTED_FILE_EXTS.contains(&other) {
                log::error!("Unsupported file format {other}");
                return Err(());
            }
            let mut content = String::with_capacity(BUFFER_CAPACITY);
            let file = File::open(path).map_err(|err| {
                log::debug!("Cannot open file {path:?} : {err}");
            })?;
            let mut file_reader = BufReader::new(file);
            file_reader.read_to_string(&mut content).map_err(|err| {
                log::debug!("Cannot read file {path:?} as UTF-8 encoded string : {err}");
            })?;

            Ok(content)
        }
    }
}
