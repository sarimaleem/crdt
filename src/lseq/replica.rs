use crate::lseq::vptr::VPtr;
use crate::message::{
    LSeqInsertOperation, LSeqInsertRequest, LSeqReadRequest, LSeqRemoveOperation,
    LSeqRemoveRequest, Message,
};
use crate::network::Network;
use crate::traits::Runnable;
use std::sync::mpsc::Receiver;
use std::sync::{atomic::AtomicBool, Arc};

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

pub struct Vertex {
    ptr: VPtr,
    value: char,
}

pub struct LSeqReplica {
    id: String,
    rx: Receiver<Message>,
    network: Network,
    running: Arc<AtomicBool>,
    vertices: Vec<Vertex>,
}

impl LSeqReplica {
    pub fn new(
        id: String,
        rx: Receiver<Message>,
        network: Network,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            id,
            rx,
            network,
            running,
            vertices: Vec::new(),
        }
    }

    fn handle_array_insert_request(&mut self, req: LSeqInsertRequest) {
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
            Message::create_lseq_insert_operation(self.id.clone(), ptr, req.value);
        self.network.broadcast_replicas(inserted_message);
    }

    fn handle_array_remove_request(&mut self, req: LSeqRemoveRequest) {
        let ptr = self.vertices[req.index].ptr.clone();
        let removed_message = Message::create_lseq_remove_operation(self.id.clone(), ptr);
        self.network.broadcast_replicas(removed_message);
    }

    fn handle_array_read_request(&mut self, req: LSeqReadRequest) {
        let result: String = self.vertices.iter().map(|s| s.value).collect();
        let response = Message::create_lseq_read_response(self.id.clone(), result);
        self.network.send_message(&req.sender_id, response);
    }

    fn handle_array_insert_operation(&mut self, op: LSeqInsertOperation) {
        let idx = match self.vertices.binary_search_by(|v| v.ptr.cmp(&op.at)) {
            Ok(i) | Err(i) => i,
        };
        self.vertices.insert(
            idx,
            Vertex {
                ptr: op.at,
                value: op.value,
            },
        );
    }

    fn handle_array_remove_operation(&mut self, op: LSeqRemoveOperation) {
        if let Ok(idx) = self.vertices.binary_search_by(|v| v.ptr.cmp(&op.at)) {
            self.vertices.remove(idx);
        }
    }
}

impl Runnable for LSeqReplica {
    fn run(&mut self) {
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    Message::LSeqInsertRequest(m) => self.handle_array_insert_request(m),
                    Message::LSeqInsertOperation(m) => self.handle_array_insert_operation(m),
                    Message::LSeqRemoveRequest(m) => self.handle_array_remove_request(m),
                    Message::LSeqRemoveOperation(m) => self.handle_array_remove_operation(m),
                    Message::LSeqReadRequest(m) => self.handle_array_read_request(m),
                    _ => panic!(),
                }
            }
        }
    }
}
