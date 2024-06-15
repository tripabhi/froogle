use std::path::Path;

pub fn read_file_to_string(path: impl AsRef<Path>) -> Result<String, ()> {
    let path = path.as_ref();
    let document = lopdf::Document::load(path).map_err(|err| {
        log::error!("Cannot load pdf file {path:?} : {err}");
    })?;

    let page_numbers = document.get_pages().keys().copied().collect::<Vec<_>>();

    let content = document.extract_text(&page_numbers).map_err(|err| {
        log::error!("Cannot parse pdf file {path:?} to string : {err}");
    })?;

    Ok(content)
}
