use std::path::Path;

const BUFFER_CAPACITY: usize = 1024;

pub fn read_file_to_string(path: impl AsRef<Path>) -> Result<String, ()> {
    let path = path.as_ref();
    let uri = format!("file://{path}", path = path.to_string_lossy());
    let document = poppler::Document::from_file(&uri, None).map_err(|err| {
        log::error!("Failed to parse pdf file {path:?} : {err}");
    })?;

    let mut content = String::with_capacity(BUFFER_CAPACITY);

    let n_pages = document.n_pages();

    for index in 0..n_pages {
        let page = document.page(index);
        match page {
            Some(page) => {
                if let Some(s) = page.text() {
                    content.push_str(s.as_str());
                    content.push(' ');
                }
            }
            None => log::error!("Page {index} is out of range of the document {path:?}"),
        }
    }

    Ok(content)
}
