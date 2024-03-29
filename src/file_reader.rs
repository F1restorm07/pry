// indexing a single file
// also index an entire directory

use unicode_segmentation::UnicodeSegmentation;
use std::{path::Path, fs::read_to_string};

#[derive(Debug)]
pub struct FileSegment<'seg>(Vec<&'seg str>);

#[derive(Debug)]
pub struct FileText<'file>(Vec<FileSegment<'file>>);

pub fn index_file<P: AsRef<Path>>(file: P) -> Vec<String> {
    let file_contents = read_to_string(file).unwrap();
    let file_contents_filtered = file_contents.to_ascii_lowercase().replace(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '\n'), " ");
    let file_segments = file_segments(&file_contents_filtered);
    let mut imp_words = Vec::new();

    for segment in file_segments {
        imp_words.extend(collect_important_words(&segment));
    }

    imp_words

}

pub fn file_segments(text: &str) -> Vec<String> {
    text
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
}

// only supports english for right now
pub fn collect_important_words(text: &str) -> Vec<String> {
    let file_language = whichlang::detect_language(text);
    let mut words: Vec<String> = vec![];
    
    let lang_stop_words = match file_language {
        whichlang::Lang::Eng => stop_words::get(stop_words::LANGUAGE::English),
        _ => vec![]
    };
    
    text
        .unicode_words()
        .filter(|w| lang_stop_words.binary_search(&w.to_string()).is_err())
        .for_each(|w| words.push(w.to_string()));

    words.sort();
    words.dedup();
    words
}
