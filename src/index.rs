use std::{collections::HashMap, hash::{Hash, SipHasher, Hasher}, path::Path};

use sled::IVec;

use crate::{query::Query, file_reader::index_file};


pub struct Index {
    words_to_file: sled::Db,
    file_ids: HashMap<String, u32>,
}

impl Index {
    pub fn new(path: &str) -> Self {
        // open db at the specified path
        Self { words_to_file: sled::open(path).unwrap(), file_ids: HashMap::new() }
    }
    // insert only important_words, will prob also need refernce to full document
    // relative path is relative to the directory the program is running in, include the extension
    pub fn insert_file(&mut self, relative_path: &str/* , words: Vec<String> */) {
        println!("inserting file: {relative_path}");
        let words = index_file(format!("./{relative_path}").as_str());
        // in the future, disallow any paths not in the current directory or its children
        // should error out potentially
        let path = relative_path.trim_start_matches("../").split_terminator('.').collect::<Vec<_>>()[0];
        let mut hasher = SipHasher::default();
        path.hash(&mut hasher);

        // hashes the file path and uses as value
        let _insert = if self.file_ids.contains_key(path) {
            None
        } else {
             self.file_ids.insert(path.to_string(), hasher.finish() as u32)
        };

        let byte_words = words.iter().map(|w| w.as_bytes()).collect::<Vec<_>>();
        let byte_words = byte_words.as_slice();

        for word in byte_words {
            let idx = self.words_to_file.get(word).unwrap();
            let file_id = self.file_ids.get(path).unwrap().to_string();

            if idx.is_some() {
                let mut new_value = idx.clone().unwrap().to_vec();

                // the hash values are 10 digits long, thus 10 bytes long
                // match the file_id with each id in the value
                if new_value.chunks(10).any(|c| c == file_id.as_bytes()) { 
                    // println!("contains: {new_value:?}");
                    break;
                } else {
                    // push the new file_id into the value
                    new_value.extend_from_slice(file_id.as_bytes());
                    let _ = self.words_to_file.insert(word, IVec::from_iter(new_value));
                }
            } else {
                let _ = self.words_to_file.insert(word, file_id.as_bytes());
            }
        }
    }
    pub fn insert_directory(&mut self, relative_path: &str) {
        let directory = Path::new(relative_path);

        if !directory.is_dir() { panic!("insert_directory directory path is not a directory") }

        for file in directory.read_dir().unwrap() {
            let file_to_insert = file.unwrap().path();
            println!("inserting_file: {file_to_insert:?} from directory: {directory:?}");
            self.insert_file(file_to_insert.to_str().unwrap());
        }
            
    }
    pub fn get_file(&self, key: &str) -> Vec<&String> {
        let file_bytes = self.words_to_file.get(key.as_bytes()).unwrap().unwrap();
        let file_ids = file_bytes.chunks(10).map(|id| String::from_utf8(id.to_vec()).unwrap().parse::<u32>().unwrap()).collect::<Vec<_>>();

        // find file from file_id (is there a better way to do this)
        self.file_ids.values().zip(self.file_ids.keys()).filter(|(val, _)| file_ids.contains(val)).map(|(_, key)| key).collect::<Vec<_>>()

    }
    // search the database, return the matching words, will also prob need to return associated files
    pub fn search(&self, query: Vec<Query>) -> Vec<String> {
        let mut db_iter = self.words_to_file.iter().keys().map(|k| k.unwrap()).collect::<Vec<_>>();

        // turn the query into bytes and match against the keys
        for item in query {
            match item {
                Query::Prefix(prefix) => {
                    db_iter.retain(|key| key[0..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes()));
                }
                Query::Suffix(suffix) => {
                    db_iter.retain(|key| key[key.len()-suffix.len()..].eq_ignore_ascii_case(suffix.as_bytes()));
                }
                Query::Exact(exact) => {
                    db_iter
                        .retain(|key| key.windows(exact.len()).any(|i| i.eq_ignore_ascii_case(exact.as_bytes())));
                }
                Query::Subsequence(subseq) => {
                    db_iter.retain(|key| {
                        let mut key_pos = 0;
                        let mut subseq_pos = 0;
                        let subseq = subseq.as_bytes();
                        while key_pos < key.len() && subseq_pos < subseq.len() {
                            if key[key_pos] == subseq[subseq_pos] {
                                subseq_pos+=1;
                            }
                            key_pos+=1;
                        }

                        subseq_pos == subseq.len()
                    });
                }
                Query::Complement(item) => {
                    match *item {
                        Query::Prefix(prefix) => {
                            db_iter.retain(|key| key[0..prefix.len()].ne(prefix.as_bytes()));
                        }
                        Query::Suffix(suffix) => {
                            db_iter.retain(|key| key[key.len()-suffix.len()..].ne(suffix.as_bytes()));
                        }
                        Query::Exact(exact) => {
                            db_iter
                                .retain(|key| key.windows(exact.len()).any(|i| i.ne(exact.as_bytes())));
                        }
                        Query::Subsequence(subseq) => {
                            db_iter.retain(|key| {
                                let mut key_pos = 0;
                                let mut subseq_pos = 0;
                                let subseq = subseq.as_bytes();
                                while key_pos < key.len() && subseq_pos < subseq.len() {
                                    if key[key_pos] == subseq[subseq_pos] {
                                        subseq_pos+=1;
                                    }
                                    key_pos+=1;
                                }

                                subseq_pos == subseq.len()
                            });
                        }
                        _ => {}
                    }
                }
                Query::Union(query) => {
                    let mut union_matched = Vec::new();
                    for item in query {
                        match item {
                            Query::Prefix(prefix) => {
                                union_matched.extend(db_iter.clone().into_iter().filter(|key| key[0..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())).collect::<Vec<_>>());
                            }
                            Query::Suffix(suffix) => {
                                union_matched.extend(db_iter.clone().into_iter().filter(|key| key[key.len()-suffix.len()..].eq_ignore_ascii_case(suffix.as_bytes())).collect::<Vec<_>>());
                            }
                            Query::Exact(exact) => {
                                union_matched.extend(db_iter.clone().into_iter()
                                    .filter(|key| key.windows(exact.len()).any(|i| i.eq_ignore_ascii_case(exact.as_bytes())))
                                    .collect::<Vec<_>>());
                            }
                            Query::Subsequence(subseq) => {
                                union_matched.extend(db_iter.clone().into_iter().filter(|key| {
                                    // let mut key_pos = Some(0);
                                    // let bytes_subseq = subseq.as_bytes();
                                    // for (idx, ch) in bytes_subseq.iter().enumerate() {
                                    //     if key_pos.is_none() { return false; }
                                    //
                                    //     // TODO: BIG FIX with multiple of same letter and letter
                                    //     // ordering
                                    //     if key[key_pos.unwrap()+1..].contains(ch) {
                                    //         key_pos = key[key_pos.unwrap()+1..]
                                    //             .iter()
                                    //             .position(|&c| 
                                    //                       c == bytes_subseq[(idx+1).min(bytes_subseq.len() - 1)]// get next character
                                    //                 );
                                    //         println!("subseq search success <{} char: {}> -> next_pos: {:?}", 
                                    //                  String::from_utf8(key.to_vec()).unwrap(), 
                                    //                  (*ch) as char,
                                    //                  key_pos);
                                    //     } else {
                                    //         return false;
                                    //     }
                                    // }
                                    // return true;

                                    let mut key_pos = 0;
                                    let mut subseq_pos = 0;
                                    let subseq = subseq.as_bytes();
                                    while key_pos < key.len() && subseq_pos < subseq.len() {
                                        if key[key_pos] == subseq[subseq_pos] {
                                            subseq_pos+=1;
                                        }
                                        key_pos+=1;
                                    }

                                    subseq_pos == subseq.len()
                                })
                                    .collect::<Vec<_>>());
                            }
                            Query::Complement(item) => {
                                match *item {
                                    Query::Prefix(prefix) => {
                                        union_matched.extend(db_iter.clone().into_iter().filter(|key| key[0..prefix.len()].ne(prefix.as_bytes())).collect::<Vec<_>>());
                                    }
                                    Query::Suffix(suffix) => {
                                        union_matched.extend(db_iter.clone().into_iter().filter(|key| key[key.len()-suffix.len()..].ne(suffix.as_bytes())).collect::<Vec<_>>());
                                    }
                                    Query::Exact(exact) => {
                                        union_matched.extend(db_iter.clone().into_iter()
                                            .filter(|key| key.windows(exact.len()).any(|i| i.ne(exact.as_bytes())))
                                            );
                                    }
                                    Query::Subsequence(subseq) => {
                                        union_matched.extend(db_iter.clone().into_iter().filter(|key| {
                                            let mut key_pos = 0;
                                            let mut subseq_pos = 0;
                                            let subseq = subseq.as_bytes();
                                            while key_pos < key.len() && subseq_pos < subseq.len() {
                                                if key[key_pos] == subseq[subseq_pos] {
                                                    subseq_pos+=1;
                                                }
                                                key_pos+=1;
                                            }

                                            subseq_pos == subseq.len()
                                        })
                                            .collect::<Vec<_>>());
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    db_iter.retain(|key| union_matched.iter().any(|i| i.eq_ignore_ascii_case(key)))
                }
            }

        }

        db_iter.iter().map(|k| String::from_utf8(k.to_vec()).unwrap()).collect::<Vec<_>>()
    }
}
