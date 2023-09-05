// use fst::Automaton;
// use fst::automaton::{ Str, Subsequence, StartsWith };

pub mod parsers;

#[derive(Debug)]
pub enum Query {
    Exact(String),
    Subsequence(String),
    Complement(Box<Self>),
    Prefix(String),
    Suffix(String),
    Union(Vec<Self>),
}

// transforms the parsed query into an intersection of automatons
// pub fn query_to_automa(query: Vec<Query>) -> impl Automaton {
//     let mut automa_query = Str::new("").complement();
//     for sect in query.iter() {
//         match sect {
//             Query::Exact(word) => automa_query.intersection(Str::new(word)),
//             Query::Subsequence(word) => automa_query.intersection(Subsequence::new(word)),
//             Query::Prefix(word) => automa_query.intersection(StartsWith(word)),
//             Query::Suffix(word) => panic!("unable to search the ends of fst sets w/o loading the entire set"),
//             Query::Complement(query) => {
//                 match query {
//                     Query::Exact(word) => automa_query.intersection(Str::new(word).complement()),
//                     Query::Subsequence(word) => automa_query.intersection(Subsequence::new(word).complement()),
//                     Query::Prefix(word) => automa_query.intersection(StartsWith::new(word).complement()),
//                     Query::Suffix(word) => panic!("unable to search the ends of fst sets w/o loading the entire set"),
//                     _ => {}
//                 }
//             }
//             Query::Union(query) => {
//                 let union_automa_query = Str::new("").complement();
//                 for sect in query.iter() {
//                     match sect {
//                         Query::Exact(word) => automa_query.union(Str::new(word)),
//                         Query::Subsequence(word) => automa_query.union(Subsequence::new(word)),
//                         Query::Prefix(word) => automa_query.union(StartsWith::new(word)),
//                         Query::Suffix(word) => panic!("unable to search the ends of fst sets w/o loading the entire set"),
//                         Query::Complement(query) => {
//                             match query {
//                                 Query::Exact(word) => automa_query.union(Str::new(word).complement()),
//                                 Query::Subsequence(word) => automa_query.union(Subsequence::new(word).complement()),
//                                 Query::Prefix(word) => automa_query.union(StartsWith::new(word).complement()),
//                                 Query::Suffix(word) => panic!("unable to search the ends of fst sets w/o loading the entire set"),
//                                 _ => {}
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//                 automa_query.intersection(union_automa_query);
//             }
//         }
//     }
//
//     automa_query
// }
