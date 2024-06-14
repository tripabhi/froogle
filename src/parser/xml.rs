use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use xml::common::Position;
use xml::reader::XmlEvent;
use xml::EventReader;

const BUFFER_CAPACITY: usize = 1024;

pub fn read_file_to_string(path: impl AsRef<Path>) -> Result<String, ()> {
    let file = File::open(path.as_ref()).map_err(|err| {
        eprintln!("Could not open {file:?} : {err}", file = path.as_ref());
    })?;

    let mut content = String::with_capacity(BUFFER_CAPACITY);
    let event_reader = EventReader::new(BufReader::new(file));

    for event_result in event_reader {
        match event_result {
            Ok(event) => {
                if let XmlEvent::Characters(s) = event {
                    content.push_str(s.as_str());
                    content.push(' '); // Add whitespace delimiter
                }
            }
            Err(err) => {
                let err_pos = err.position();
                eprintln!(
                    "Error while parsing xml file {file:?} at [{row}, {col}] : {err}",
                    file = path.as_ref(),
                    row = err_pos.row,
                    col = err_pos.column
                );
            }
        }
    }

    Ok(content)
}
