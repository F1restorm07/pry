// #![feature(test)]

use std::fs::read_to_string;

// use std::{fs::File, path::Path};
// use fst::{ Streamer, IntoStreamer };
// use memmap2::Mmap;
// use document::Document;
use pry::{query, index::Index};
use combine::{EasyParser, Parser};

// const COLLECTION_POOL_PATH: &'static str = "./collection";

fn main() {
    let test_query = query::parsers::parse_query().easy_parse("'cyr | ^f | !h 'ky").unwrap();
    println!("{test_query:?}");
    let test_query = query::parsers::parse_query().easy_parse("^f g$ 'tt|et").unwrap();
    println!("{:?}", test_query);

    let imp_words = pry::file_reader::collect_important_words("./gutenburg/12374.txt");
    // println!("{imp_words:?}");
    let mut db = Index::new("collections");
    db.insert_file("gutenburg/12374", imp_words);
    db.insert_file("readme.md", pry::file_reader::collect_important_words("./readme.md"));
    let matched = db.search(test_query.0);
    println!("{matched:?}");

    for word in matched {
        println!("{word}: {:?}", String::from_utf8(db.get(&word).unwrap().unwrap().to_vec()).unwrap());
    }
    // println!("{:?}", db.words_to_file.get(b"vanished"));
}
