use self::lexer::Lexer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::usize;

pub mod cache;
pub mod lexer;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Model {
    pub(crate) doc_store: HashMap<PathBuf, Doc>,
    pub(crate) doc_freq: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Doc {
    term_freq: HashMap<String, usize>,
    max_term_freq: usize,
    n_terms: usize,
    last_modified: SystemTime,
}

impl Model {
    pub fn should_index(&self, path: &Path, last_modified: SystemTime) -> bool {
        if let Some(doc) = self.doc_store.get(path) {
            return doc.last_modified < last_modified;
        }
        true
    }

    pub fn remove_document(&mut self, path: &Path) {
        if let Some(doc) = self.doc_store.remove(path) {
            for key in doc.term_freq.keys() {
                if let Some(freq) = self.doc_freq.get_mut(key) {
                    *freq -= 1;
                }
            }
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), ()> {
        let file = File::create(path).map_err(|err| {
            log::error!("Cannot create file {path:?} : {err}");
        })?;

        serde_json::to_writer(BufWriter::new(file), &self).map_err(|err| {
            log::error!("Cannot serialize model to file : {err}");
        })?;

        Ok(())
    }

    pub fn search_document(&self, query: &str) -> Vec<(PathBuf, f32)> {
        let query = query.chars().collect::<Vec<_>>();
        let query_tokens = Lexer::new(&query).collect::<Vec<_>>();

        let mut results = Vec::new();
        for (path, doc) in &self.doc_store {
            let mut rank = 0_f32;
            let mut token_freq_sum = 0_usize;
            for token in &query_tokens {
                rank += tf(token, doc) * idf(token, self.doc_store.len(), &self.doc_freq);

                token_freq_sum += doc.term_freq.get(token).copied().unwrap_or(0);
            }

            if token_freq_sum > 0 {
                results.push((path.clone(), rank));
            }
        }

        results.sort_by(|(_, rank1), (_, rank2)| {
            rank2.partial_cmp(&rank1).expect("Rank comparison failed")
        });

        results
    }

    pub fn add_document(
        &mut self,
        file_path: impl AsRef<Path>,
        last_modified: SystemTime,
        content: &[char],
    ) {
        let mut term_freq: HashMap<String, usize> = HashMap::new();
        let mut n_terms = 0;
        let mut max_term_freq = 1;

        for token in Lexer::new(content) {
            if let Some(freq) = term_freq.get_mut(&token) {
                *freq += 1;
                if max_term_freq < *freq {
                    max_term_freq = *freq;
                }
            } else {
                term_freq.insert(token, 1);
            }
            n_terms += 1;
        }

        for token in term_freq.keys() {
            if let Some(freq) = self.doc_freq.get_mut(token) {
                *freq += 1;
            } else {
                self.doc_freq.insert(token.to_owned(), 1);
            }
        }

        self.doc_store.insert(
            file_path.as_ref().to_path_buf(),
            Doc {
                term_freq,
                max_term_freq,
                n_terms,
                last_modified,
            },
        );
    }
}

pub(crate) fn tf(term: &str, document: &Doc) -> f32 {
    let max_term_freq = document.max_term_freq as f32;
    let term_freq = document.term_freq.get(term).copied().unwrap_or(0) as f32;
    0.5 + (0.5) * (term_freq / max_term_freq)
}

pub(crate) fn idf(term: &str, corpus_size: usize, doc_freq: &HashMap<String, usize>) -> f32 {
    let freq_in_doc = doc_freq.get(term).copied().unwrap_or(0) as f32;
    (corpus_size as f32 / (1.0 + freq_in_doc)).log10() + 1.0
}
