use crate::message::{
    ArrayInsertOperation, ArrayInsertRequest, ArrayRemoveOperation, ArrayRemoveRequest, Message,
};
use crate::network::Network;
use crate::traits::Runnable;
use std::sync::mpsc::Receiver;
use std::sync::{atomic::AtomicBool, Arc};

use std::cmp::Ordering;

pub struct VPtr {
    sequence: Vec<u8>,
    id: String,
}

impl VPtr {
    pub fn new(sequence: Vec<u8>, id: String) -> Self {
        Self { sequence, id }
    }
}

pub struct Vertex {
    ptr: VPtr,
    value: char,
}

impl Clone for VPtr {
    fn clone(&self) -> Self {
        Self {
            sequence: self.sequence.clone(),
            id: self.id.clone(),
        }
    }
}

impl Ord for VPtr {
    fn cmp(&self, other: &Self) -> Ordering {
        let len = std::cmp::min(self.sequence.len(), other.sequence.len());
        for i in 0..len {
            let cmp = self.sequence[i].cmp(&other.sequence[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        if self.sequence.len() != other.sequence.len() {
            return self.sequence.len().cmp(&other.sequence.len());
        }

        self.id.cmp(&other.id)
    }
}

impl PartialOrd for VPtr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for VPtr {}

impl PartialEq for VPtr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn generate_seq(lo: &[u8], hi: &[u8]) -> Vec<u8> {
    let mut acc: Vec<u8> = Vec::new();
    let mut i = 0;

    loop {
        let min = if i >= lo.len() { 0 } else { lo[i] };
        let max = if i >= hi.len() { 255 } else { hi[i] };

        if min + 1 < max {
            acc.push(min + 1);
            return acc;
        } else {
            acc.push(min);
        }

        i += 1;
    }
}

pub struct ArrayReplica {
    id: String,
    rx: Receiver<Message>,
    network: Network,
    running: Arc<AtomicBool>,
    vertices: Vec<Vertex>,
}

impl ArrayReplica {
    fn handle_array_insert_request(&mut self, req: ArrayInsertRequest) {
        let left = if req.index == 0 {
            Vec::new()
        } else {
            self.vertices[req.index - 1].ptr.sequence.clone()
        };
        let right = if req.index == self.vertices.len() {
            Vec::new()
        } else {
            self.vertices[req.index].ptr.sequence.clone()
        };
        let ptr = VPtr::new(generate_seq(&left, &right), self.id.clone());
        let inserted_message =
            Message::create_array_insert_operation(self.id.clone(), ptr, req.value);
        self.network.broadcast_replicas(inserted_message);
    }

    fn handle_array_remove_request(&mut self, req: ArrayRemoveRequest) {
        let ptr = self.vertices[req.index].ptr.clone();
        let removed_message = Message::create_array_remove_operation(self.id.clone(), ptr);
        self.network.broadcast_replicas(removed_message);
    }

    fn handle_array_insert_operation(&mut self, op: ArrayInsertOperation) {
        // TODO check if this is correct, I'm not sure how the binary search works, and I'd want it
        // to return the less than greater than value right?
        let mut idx = match self.vertices.binary_search_by(|v| v.ptr.cmp(&op.at)) {
            Ok(i) | Err(i) => i,
        };
        if self.vertices[idx].ptr < op.at {
            idx += 1;
        }
        self.vertices.insert(
            idx,
            Vertex {
                ptr: op.at,
                value: op.value,
            },
        );
    }

    fn handle_array_remove_operation(&mut self, op: ArrayRemoveOperation) {
        if let Ok(idx) = self.vertices.binary_search_by(|v| v.ptr.cmp(&op.at)) {
            self.vertices.remove(idx);
        }
    }
}

impl Runnable for ArrayReplica {
    fn run(&mut self) {
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    Message::ArrayInsertRequest(m) => self.handle_array_insert_request(m),
                    Message::ArrayInsertOperation(m) => self.handle_array_insert_operation(m),
                    Message::ArrayRemoveRequest(m) => self.handle_array_remove_request(m),
                    Message::ArrayRemoveOperation(m) => self.handle_array_remove_operation(m),
                    _ => panic!(),
                }
            }
        }
    }
}
