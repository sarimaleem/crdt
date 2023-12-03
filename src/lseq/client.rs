use crate::message::CounterReadResult;
use crate::network::Network;
use rand::{distributions::Alphanumeric, Rng};
use std::sync::{mpsc::Receiver, atomic::AtomicBool};
use std::sync::Arc;

use crate::{message::Message, traits::Runnable};

pub struct LSeqClient {
    id: String,
    n_requests: i32,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
}

impl LSeqClient {
    pub fn new(id: String, n_requests: i32, network: Network, assigned_replica_id: String, rx: Receiver<Message>, running: Arc<AtomicBool>) -> Self { Self { id, n_requests, network, assigned_replica_id, rx, running } }
}

impl Runnable for LSeqClient {
    fn run(&mut self) {
        // TODO:
        // current pseudocode implementation
        // send a read request, get the length
        // send 100 insert requests of random characters at random indicies
        // send 5 remove requests of random characters
        // should I wait for acks?
        //
        // FIXME: add delete requests, for now let's just do insertion
        for _ in 0..self.n_requests {
            // read request
            let read_req = Message::create_lseq_read_request(self.id.clone());
            self.network.send_message(&self.assigned_replica_id, read_req);
            let text_length;
            let response = self.rx.recv().unwrap();
            match response {
                Message::LSeqReadResponse(m) => text_length = m.result.len(),
                _ => panic!(),
            };

            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..text_length);
            let value: char = rng.sample(Alphanumeric) as char;
            let insert_request = Message::create_lseq_insert_request(self.id.clone(), index, value);
            self.network.send_message(&self.assigned_replica_id, insert_request);
        }
    }
}
