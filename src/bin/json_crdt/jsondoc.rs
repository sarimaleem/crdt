/// a struct that would store the local doc.
use serde_json::{ Value};
use std::fmt;
use std::path::Component::ParentDir;
use crate::op::Op;

#[derive(Debug)]
/// This is struct that would hold the actual json document,
pub struct JSONDoc {
    /// the lamport time of this JSONDoc
    clock: u32,
    /// the id of the doc
    pub id: u32,
    /// Doc representation using serde_json
    pub doc: Value,
}

impl JSONDoc {
    pub fn new(self, id: u32) -> JSONDoc {
        JSONDoc {
            id: (id),
            clock: (0),
            doc: Value::Object(Default::default()),
        }
    }

    pub fn execute(self, operations: &Vec<Op>) {
        for operation in operations {
            if operation.id.node_id == self.id {
                self.apply_op(operation);
                self.broadcast_op(operation);
            } else {

            }
        }
    }

    pub fn apply_op(self, operation: &Op) {

    }

    pub fn broadcast_op(self, operation: &Op) {

    }
}

impl fmt::Display for JSONDoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the Display trait
        write!(f, "{}", self.doc)
    }
}