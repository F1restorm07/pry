use crate::operation::IndexOperation;

use super::Event;

#[derive(Debug)]
pub struct IndexOpEvent {
    pub op: IndexOperation
}

impl Event for IndexOpEvent {}

impl IndexOpEvent {
    pub fn new(op: IndexOperation) -> Self {
        Self { op }
    }
}
