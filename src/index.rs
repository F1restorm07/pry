use std::{path::Path, io::BufRead, future::Future, error::Error, pin::Pin, task::{Context, Poll}};
use uuid::Uuid;

use sled::IVec;

use crate::{query::Query, file_reader::index_file, event::{EventSubscriber, index::IndexOpEvent, Event, user::UserOpEvent}, operation::{UserOperation, IndexOperation}};


#[derive(Debug)] // will need to remove clone later
pub struct Index {
    // words_to_file: sled::Db,
    // file_ids: HashMap<String, u32>,
    db: sled::Db, // database containing everything
    file_to_id: sled::Tree, // file to file_id
    file_to_metadata: sled::Tree, // file to file_metadata
    tags: sled::Tree, // collection of all tags (contain files and directories)
    words_to_file: sled::Tree, // individual words linked to all of their respective files
    file_to_words: sled::Tree, // flies to all of their contained words
}

impl EventSubscriber for Index {
    fn receive_event(self: &Index, event: &dyn crate::event::Event) {
        println!("[Index::receive_event] incoming event: {event:?}");
        // downcasting to UserOpEvent from &dyn Event
        let event: &UserOpEvent = event.downcast_ref::<UserOpEvent>().unwrap();
        match &event.op {
            // how to get rid of the clones 
            UserOperation::Insert{ file_name, words, byte_words } => {
                assert!(
                    !self.file_to_id.contains_key(file_name.clone()).unwrap(),
                    "the file has already been inserted into the engine index, consider updating it instead"
                    );

                let uuid = Uuid::new_v4();
                let byte_uuid = uuid.as_bytes();
                self.file_to_words.insert(file_name.clone(), byte_words.to_vec());
                self.file_to_id.insert(file_name, byte_uuid);

                // let words = words.iter().map(|w| w.as_bytes()).collect::<Vec<_>>();
                for word in words {
                    let idx = self.words_to_file.get(word.to_vec()).unwrap();

                    if idx.is_some() {
                        let mut new_value = idx.clone().unwrap().to_vec();

                        // the uuid values are 16 digits long, thus 16 bytes long
                        // match the file_id with each id in the value
                        if new_value.chunks(16).any(|c| c == byte_uuid) { 
                            // println!("contains: {new_value:?}");
                            break;
                        } else {
                            // push the new file_id into the value
                            new_value.extend_from_slice(byte_uuid);
                            let _ = self.words_to_file.insert(word, IVec::from_iter(new_value));
                        }
                    } else {
                        let _ = self.words_to_file.insert(word, byte_uuid);
                    }
                }

            }
            // how to return matched words 
            UserOperation::Query(query) => {
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
                            match **item {
                                Query::Prefix(ref prefix) => {
                                    db_iter.retain(|key| key[0..prefix.len()].ne(prefix.as_bytes()));
                                }
                                Query::Suffix(ref suffix) => {
                                    db_iter.retain(|key| key[key.len()-suffix.len()..].ne(suffix.as_bytes()));
                                }
                                Query::Exact(ref exact) => {
                                    db_iter
                                        .retain(|key| key.windows(exact.len()).any(|i| i.ne(exact.as_bytes())));
                                }
                                Query::Subsequence(ref subseq) => {
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

                                        subseq_pos != subseq.len()
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
                                        let subseq = subseq.as_bytes();
                                        union_matched.extend(db_iter.clone().into_iter().filter(|key| {
                                            let mut key_pos = 0;
                                            let mut subseq_pos = 0;
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
                                        match **item {
                                            Query::Prefix(ref prefix) => {
                                                union_matched.extend(db_iter.clone().into_iter().filter(|key| key[0..prefix.len()].ne(prefix.as_bytes())).collect::<Vec<_>>());
                                            }
                                            Query::Suffix(ref suffix) => {
                                                union_matched.extend(db_iter.clone().into_iter().filter(|key| key[key.len()-suffix.len()..].ne(suffix.as_bytes())).collect::<Vec<_>>());
                                            }
                                            Query::Exact(ref exact) => {
                                                union_matched.extend(db_iter.clone().into_iter()
                                                    .filter(|key| key.windows(exact.len()).any(|i| i.ne(exact.as_bytes())))
                                                    );
                                            }
                                            Query::Subsequence(ref subseq) => {
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

                                                    subseq_pos != subseq.len()
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

                IndexOpEvent::new(IndexOperation::matched(db_iter.iter().map(|v| v.to_vec()).collect())).dispatch("user");
            }
            _ => {}
        }
    }
}

impl Index {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).unwrap();
        let file_to_id = db.open_tree("file_to_id").unwrap();
        let file_to_metadata = db.open_tree("file_to_metadata").unwrap();
        let tags = db.open_tree("tags").unwrap();
        let words_to_file = db.open_tree("words_to_file").unwrap();
        let file_to_words = db.open_tree("file_to_words").unwrap();
        Self {
            db,
            file_to_id,
            file_to_metadata,
            tags,
            words_to_file,
            file_to_words,
        }
    }
    // insert only important_words, will prob also need refernce to full document
    // relative path is relative to the directory the program is running in, include the extension
    pub fn insert_directory(&mut self, relative_path: &str) {
        let directory = Path::new(relative_path);

        if !directory.is_dir() { panic!("[Index::insert_directory] directory path is not a directory") }

        for file in directory.read_dir().unwrap() {
            let file_to_insert = file.unwrap().path();
            println!("inserting_file: {file_to_insert:?} from directory: {directory:?}");
        }
            
    }
    pub fn get_file(&self, key: &str) -> Vec<String> {
        self
            .file_to_words
            .iter()
            .filter(|kv| kv.as_ref().unwrap().1.split(|c| c == &b'/').any(|v| v == key.as_bytes()))
            .map(|kv| String::from_utf8(kv.unwrap().0.to_vec()).unwrap())
            .collect::<Vec<_>>()
    }
}
