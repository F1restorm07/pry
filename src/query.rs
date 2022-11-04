use logos::{ Logos, Span, Lexer, Filter };
use fst::{ self, Streamer, IntoStreamer };
use std::fs::File;
use crate::operation::{ Operation, tokenize };
use crate::document::Index;

pub struct Query<'q> {
    operation: Operation,
    index: Index<'q>,
    // filters: Filters,
}

pub fn search(query: &str) {
    let mem_map = unsafe { 
        memmap::Mmap::map(&File::open(format!("{}/.cache/map.fst", env!("HOME"))).unwrap()).unwrap()
    };

    let set = fst::Set::new(mem_map).unwrap();
    let set_strs = set.stream().into_strs();
    // let rev_set = set_strs.iter().rev();
    // let mut fst = set.as_fst().stream();
    
    // println!("{rev_set:?}");
    // println!("{:?}", fst::Set::new(fst));
    // println!("{query:#?}");
    // let query = Query::new(query);
    // let tokens = tokenize(query);
    // let automaton = ();
    // let mut stream = query.build_stream(&set);
    
    // while let Some(entries) = stream.next() {
        // println!("{:?}", String::from_utf8(entries.to_vec()));
        // for e in entries {
        //     println!("{e}");
        // }
    // };

}
