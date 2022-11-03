pub mod operation;
pub mod document;
pub mod query;
pub mod index;

use std::fs::File;
use  fst::{ Streamer, IntoStreamer };

fn main() {
    index::index(&mut std::fs::File::open("Cargo.toml").unwrap());
    // query::search("'rkyv");
    let op = operation::Operation::build(operation::tokenize("s | r"));
    println!("{op:?}");
    // println!("{:?}", operation::Operation::automate(op));
    let automatons = operation::Operation::automate(op);
    let mem_map = unsafe {
        memmap::Mmap::map(&File::open(format!("{}/.cache/map.fst", env!("HOME"))).unwrap()).unwrap()
    };
    let set = fst::Set::new(mem_map).unwrap();
    let mut ops = fst::set::OpBuilder::new();

    for automaton in automatons.iter() {
        match automaton {
            operation::AutomatonKind::Str(dfa) => ops.push(set.search(dfa)),
            operation::AutomatonKind::Subsequence(dfa) => ops.push(set.search(dfa)),
            operation::AutomatonKind::Prefix(dfa) => ops.push(set.search(dfa)),
            operation::AutomatonKind::Inverse(dfa) => ops.push(set.search(dfa)),
            operation::AutomatonKind::Or(autos) => {
                let mut or_ops = fst::set::OpBuilder::new();
                for auto in autos {
                    match auto {
                        operation::AutomatonKind::Str(dfa) => or_ops.push(set.search(dfa)),
                        operation::AutomatonKind::Subsequence(dfa) => or_ops.push(set.search(dfa)),
                        operation::AutomatonKind::Prefix(dfa) => or_ops.push(set.search(dfa)),
                        operation::AutomatonKind::Inverse(dfa) => or_ops.push(set.search(dfa)),
                        _ => {}
                   }
                }
                ops.push(or_ops.union())
            }
        };
    }

    let mut query = ops.intersection().into_stream();

    while let Some(entry) = query.next() {
        println!("{:?}", String::from_utf8(entry.to_vec()));
    }

}
