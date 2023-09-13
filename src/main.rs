use std::sync::OnceLock;

use pry::{index::Index, cli::{create_cli_conf, CmdOptionsConfig}, event::{Event, EventDispatcher, user::{UserEventer, UserOpEvent}}, operation::UserOperation};

static DB: OnceLock<Index> = OnceLock::new();

fn main() {
    DB.get_or_init(|| Index::new("collections"));
    run(&create_cli_conf().unwrap());
}

fn run(conf: &CmdOptionsConfig) {
    let dispatcher = EventDispatcher::new();
    dispatcher.add_route("index", DB.get().unwrap());
    dispatcher.add_route("user", &UserEventer {});

    if conf.subcmd_conf.subcmd_insert {
        println!("input file: {}", conf.subcmd_conf.flag_file);
        UserOpEvent::new(UserOperation::insert(conf.subcmd_conf.flag_file.as_str())).dispatch("index");
    } else if conf.subcmd_conf.subcmd_query {
        println!("input query: {}", conf.subcmd_conf.flag_query);
       UserOpEvent::new(UserOperation::query(conf.subcmd_conf.flag_query.as_str())).dispatch("index"); 
    }
}

