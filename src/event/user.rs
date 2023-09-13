use super::{Event, EventSubscriber, index::IndexOpEvent};
use crate::operation::{UserOperation, IndexOperation};

#[derive(Debug)]
pub struct UserOpEvent {
    pub op: UserOperation,
}

impl UserOpEvent {
    pub fn new(op: UserOperation) -> Self {
        Self { op }
    }
}

impl Event for UserOpEvent {}

#[derive(Debug)]
pub struct UserEventer;

impl EventSubscriber for UserEventer {
    fn receive_event(&self, event: &dyn Event) {
        let event = event.downcast_ref::<IndexOpEvent>().unwrap();

        match &event.op {
            IndexOperation::Matched(words) => println!("{words:?}"),
        }
    }
}
