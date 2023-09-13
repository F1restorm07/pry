// use std::{path::Path, convert::Infallible, borrow::BorrowMut, cell::RefCell};

use crate::query::Query;
use crate::file_reader::index_file;
use crate::query::parsers::parse_query;

use combine::Parser;

#[derive(Debug)]
pub enum UserOperation {
    Insert {
        file_name: Vec<u8>,
        words: Vec<Vec<u8>>,
        byte_words: Vec<u8>
    }, // insert a single file (relative path), add support for directories later
    Query(Vec<Query>),
    Update(String),
    Delete(String)
}

impl UserOperation {
    pub fn insert(relative_path: &str) -> Self {
        println!("inserting file: {relative_path}");
        let words = index_file(format!("./{relative_path}").as_str());
        // in the future, disallow any paths not in the current directory or its children
        // should error out potentially
        let path = relative_path.trim_start_matches("../").split_terminator('.').collect::<Vec<_>>()[0];

        // will remove to_vec() in the future
        let byte_words = words.iter().map(|w| w.as_bytes().to_vec()).collect::<Vec<_>>();
        // let byte_words = byte_words.as_slice();

        let delim_words = words.iter().map(|w| {
            let mut bw = w.clone().into_bytes();
            bw.push(b'/');
            bw
        })
        .flatten()
        .collect::<Vec<_>>();

        Self::Insert { file_name: path.as_bytes().to_vec(), words: byte_words, byte_words: delim_words }
    }
    pub fn query(query: &str) -> Self {
        Self::Query(parse_query().parse(query).unwrap().0)
    }
}

#[derive(Debug)]
pub enum IndexOperation {
    Matched(Vec<String>),
}

impl IndexOperation {
    pub fn matched(words: Vec<Vec<u8>>) -> Self {
        Self::Matched(words.into_iter().map(|w| String::from_utf8(w).unwrap()).collect())
    }
}
