pub mod search;
pub mod query;
pub mod index;

fn main() {
    // let index = index::index(&mut std::fs::File::open("Cargo.toml").unwrap());
    query::search("'rkyv");
}
