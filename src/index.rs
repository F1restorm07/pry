use std::io::{BufReader, BufWriter, prelude::*};
use std::fs::File;

pub fn index(file: &mut File) /* -> Result<Set<memmap::Mmap>, fst::Error> */ {
    // let reader = dir.read_dir().expect("cannot read directory or file");
    let mut contents = String::new();
    let mut reader = BufReader::new(file);

    // let writer = BufWriter::new(File::create(format!("{}/.cache/map.fst", env!("HOME"))).unwrap());
    let writer = BufWriter::new(File::create("map.fst").unwrap());

    reader.read_to_string(&mut contents).unwrap();

    // only take the words and numbers, keep apostrophes
    let words = contents.replace(|c: char| !(c.is_alphanumeric() || c == '\''), " ");
    // split and sort the words and numbers
    let mut sort_contents: Vec<&str> = words
        .split_whitespace()
        .filter(|&s| !s.trim().is_empty())
        .collect();
    sort_contents.sort();

    // write to the specified file
    let mut build = fst::SetBuilder::new(writer).unwrap();
    build.extend_iter(sort_contents).unwrap();
    build.finish().expect("could not write index to file");
    // println!("{}", include_bytes!("map.fst"));

}
