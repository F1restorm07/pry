use std::{collections::HashMap, hash::{Hash, SipHasher, Hasher}};

use sled::IVec;

use crate::query::Query;


pub struct Index {
    pub words_to_file: sled::Db,
    file_ids: HashMap<String, u32>,
}

impl Index {
    pub fn new(path: &str) -> Self {
        // open db at the specified path
        Self { words_to_file: sled::open(path).unwrap(), file_ids: HashMap::new() }
    }
    // insert only important_words, will prob also need refernce to full document
    pub fn insert_file(&mut self, path: &str, words: Vec<String>) {
        let mut hasher = SipHasher::default();
        path.hash(&mut hasher);

        // hashes the file path and uses as value
        let _insert = if self.file_ids.contains_key(path) {
            None
        } else {
             self.file_ids.insert(path.to_string(), hasher.finish() as u32)
        };
        println!("file_ids: {:?}", self.file_ids);

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
                    // println!("new: {new_value:?}");
                    self.words_to_file.insert(word, IVec::from_iter(new_value));
                }
            } else {
                self.words_to_file.insert(word, file_id.as_bytes());
                // println!("new_word: {} -> {:?}",String::from_utf8(word.to_vec()).unwrap(), self.words_to_file.get(word));
            }
        }
    }
    pub fn get(&self, key: &str) -> Result<Option<IVec>, sled::Error> {
        self.words_to_file.get(key.as_bytes())
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
                        for char in subseq.as_bytes() {
                            if !key.contains(char) {
                                return false;
                            } else {
                                continue;
                            }
                        }
                        return true;
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
                                for char in subseq.as_bytes() {
                                    if !key.contains(char) {
                                        return true;
                                    } else {
                                        continue;
                                    }
                                }
                                return false;
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
                                    for char in subseq.as_bytes() {
                                        if !key.contains(char) {
                                            return false;
                                        } else {
                                            continue;
                                        }
                                    }
                                    return true;
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
                                            for char in subseq.as_bytes() {
                                                if !key.contains(char) {
                                                    return true;
                                                } else {
                                                    continue;
                                                }
                                            }
                                            return false;
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
