use html5gum::Token;
use html5gum::Tokenizer;
use std::fs::File;
use std::path::Path;

static ESCAPE_TAGS: [&str; 2] = ["script", "style"];
const BUFFER_CAPACITY: usize = 1024;

fn is_escape_tag(other: impl AsRef<str>) -> bool {
    let other = other.as_ref();
    for tag in ESCAPE_TAGS {
        if tag.eq(other) {
            return true;
        }
    }
    false
}

pub fn read_file_to_string(path: impl AsRef<Path>) -> Result<String, ()> {
    let file = File::open(path.as_ref()).map_err(|err| {
        eprintln!("Error while opening {file:?} : {err}", file = path.as_ref());
    })?;

    let mut content = String::with_capacity(BUFFER_CAPACITY);
    let mut escape_str = false;

    for token_result in Tokenizer::new(file) {
        match token_result {
            Ok(token) => match token {
                Token::StartTag(tag) => {
                    if is_escape_tag(String::from_utf8_lossy(&tag.name)) && !escape_str {
                        escape_str = true;
                    }
                }
                Token::EndTag(tag) => {
                    if is_escape_tag(String::from_utf8_lossy(&tag.name)) && escape_str {
                        escape_str = false;
                    }
                }
                Token::String(s) => {
                    if !escape_str {
                        content.push_str(String::from_utf8_lossy(&s).as_ref());
                        content.push(' '); // Add whitespace delimiter
                    }
                }
                _ => {}
            },
            Err(err) => eprintln!(
                "Error while parsing html file {file:?} : {err}",
                file = path.as_ref()
            ),
        }
    }

    Ok(content)
}
