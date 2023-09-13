use pry::{index::Index, event::{Event, EventDispatcher, user::{UserEventer, UserOpEvent}}, operation::UserOperation};
use std::sync::OnceLock;

static DB: OnceLock<Index> = OnceLock::new();

fn main() {
    DB.get_or_init(|| Index::new("bench"));
    let dispatcher = EventDispatcher::new();
    dispatcher.add_route("index", DB.get().unwrap());
    dispatcher.add_route("user", &UserEventer {});

    UserOpEvent::new(UserOperation::insert("gutenburg/12374.txt")).dispatch("index");

    brunch::benches!(
        inline:
        // brunch::Bench::new("dispatch_index_file_insert")
        //     .with_samples(1000)
        //     .run(dispatcher)
        brunch::Bench::new("dispatch_index_query")
            .run(|| UserOpEvent::new(UserOperation::query("ban")).dispatch("index"))
                    );
}
