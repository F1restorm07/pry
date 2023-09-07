use pry::{query, index::Index};
use combine::EasyParser;

fn main() {
    let test_query = query::parsers::parse_query().easy_parse("apple | ban").unwrap();
    println!("{test_query:?}");

    let mut db = Index::new("collections");
    db.insert_file("gutenburg/12374.txt");
    db.insert_file("readme.md");
    db.insert_directory("../../../learning");
    let matched = db.search(test_query.0);
    println!("{matched:?}");

    for word in matched {
        let file = db.get_file(word.as_str());
        println!("{word}: {file:?}");
    }
}
