// indexing a single file
// also index an entire directory

use std::path::Path;
use std::fs::{self, File};

pub fn index_file(path: &Path) {
    let reader = fs::read(path);

    let words = reader;
}
