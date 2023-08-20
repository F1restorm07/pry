// #![feature(test)]

// use std::{fs::File, path::Path};
// use fst::{ Streamer, IntoStreamer };
// use memmap2::Mmap;
// use document::Document;
use pry::query;
use combine::EasyParser;

// const COLLECTION_POOL_PATH: &'static str = "./collection";

fn main() {
    // index::index(&mut File::open(&"./gutenburg/12374.txt").unwrap());
    // let op = operation::Operation::build(operation::tokenize("fc | !'tu ^s"));
    // let pool = document::IndexPool::acuqire("c:test:1", "d:test:1");
    // let i = pool.map_err(|err| format!("{err:?}")).unwrap();
    // println!("{:?}", i.fst.stream().into_str_vec());

    // let op = operation::Operation::build(operation::tokenize("fr"));
    // let filter = filter::Filter::build(filter::tokenize("test > test2 test3 <= test4"));
    // let automatons = op.automate();

    // let json: serde_json::Value = serde_json::from_str(std::fs::read_to_string("../frontend/package.json").unwrap().as_str()).unwrap();

    // let mut set = fst::raw::Fst::from_iter_set(std::io::BufReader::new(File::open("map.fst").unwrap()).buffer().as_ref());
    // let mut set = fst::raw::Builder::new();
    // let mut set = fst::raw::Builder::memory();
    // let index = unsafe { Mmap::map(&File::open("map.fst").unwrap()).unwrap() };
    // let set = fst::raw::Fst::new(index).unwrap();

    // set.insert(json["name"].as_str().unwrap(), 1010101).unwrap();

    // let set = set.into_fst();
    // let mut ops = fst::raw::OpBuilder::new();
    let test_query = query::parse_query().easy_parse("cyr | ^f | !h 'ky").unwrap();
    println!("{test_query:?}");

    // for automaton in automatons.iter() {
    //     match automaton {
    //         operation::AutomatonKind::Str(dfa) => ops.push(set.search(dfa)),
    //         operation::AutomatonKind::Subsequence(dfa) => ops.push(set.search(dfa)),
    //         operation::AutomatonKind::Prefix(dfa) => ops.push(set.search(dfa)),
    //         operation::AutomatonKind::Inverse(dfa) => ops.push(set.search(dfa)),
    //         operation::AutomatonKind::Or(dfa) => ops.push(set.search(dfa)),
    //     };
    // }

    // let mut query = ops.intersection();
    //
    // while let Some(entry) = query.next() {
    //     println!("{:?}", String::from_utf8(entry.0.to_vec()));
    //     // println!("state {:?}", entry.1);
    // }

}

// mod benches {
//     extern crate test;
//
//     use super::*;
//
//     #[bench]
//     fn bench_search_time(b: &mut test::Bencher) {
//         b.iter(|| {
//         let mem_map = unsafe {
//             Mmap::map(&File::open("map.fst").unwrap()).unwrap()
//         };
//         let automatons = operation::Operation::build(operation::tokenize("cyr | ^f | !h 'ky")).automate();
//         let set = fst::raw::Fst::new(mem_map).unwrap();
//         let mut ops = fst::raw::OpBuilder::new();
//
//         for automaton in automatons.iter() {
//             match automaton {
//                 operation::AutomatonKind::Str(dfa) => ops.push(set.search(dfa)),
//                 operation::AutomatonKind::Subsequence(dfa) => ops.push(set.search(dfa)),
//                 operation::AutomatonKind::Prefix(dfa) => ops.push(set.search(dfa)),
//                 operation::AutomatonKind::Inverse(dfa) => ops.push(set.search(dfa)),
//                 operation::AutomatonKind::Or(dfa) => ops.push(set.search(dfa)),
//             };
//         }
//
//         let mut query = ops.intersection().into_stream();
//         });
//     }
//
//     #[bench]
//     fn bench_operation_time(b: &mut test::Bencher) {
//         b.iter(|| {
//             operation::Operation::build(operation::tokenize("cyr | ^f | !h 'ky")).automate();
//         })
//     }
//
//     #[bench]
//     fn bench_filter_time(b: &mut test::Bencher) {
//         b.iter(|| {
//             filter::Filter::build(filter::tokenize("time > 10:10 name = test"));
//         })
//     }
//
//     #[bench]
//     fn bench_index_acquire_time(b: &mut test::Bencher) {
//         b.iter(|| {
//             document::IndexPool::acuqire("c:test:1", "d:test:1"); // c -> collection, d -> document
//         })
//     }
// }
