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
