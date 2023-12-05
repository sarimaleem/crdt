use crate::network::Network;
use rand::{distributions::Alphanumeric, Rng};
use std::sync::{atomic::AtomicBool, mpsc::Receiver};
use std::sync::{Arc, Barrier};

use crate::{message::Message, traits::Runnable};

pub struct LSeqClient {
    id: String,
    n_requests: usize,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
    barrier: Arc<Barrier>,
    output: bool,
}

impl LSeqClient {
    pub fn new(
        id: String,
        n_requests: usize,
        network: Network,
        assigned_replica_id: String,
        rx: Receiver<Message>,
        running: Arc<AtomicBool>,
        barrier: Arc<Barrier>,
        output: bool,
    ) -> Self {
        Self {
            id,
            n_requests,
            network,
            assigned_replica_id,
            rx,
            running,
            barrier,
            output,
        }
    }
}

impl Runnable for LSeqClient {
    fn run(&mut self) {
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
            self.network
                .send_message(&self.assigned_replica_id, read_req);
            let text_length;
            let response = self.rx.recv().unwrap();
            match response {
                Message::LSeqReadResponse(m) => text_length = m.result.len(),
                _ => panic!(),
            };

            // insertions
            for _ in 0..10 {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..=text_length);
                let value: char = rng.sample(Alphanumeric) as char;
                let insert_request = Message::create_lseq_insert_request(self.id.clone(), index, value);
                self.network
                    .send_message(&self.assigned_replica_id, insert_request);
            }

            // deletions
            for _ in 0..2 {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..=text_length);
                let delete_request = Message::create_lseq_remove_request(self.id.clone(), index);
                self.network
                    .send_message(&self.assigned_replica_id, delete_request);
            }
        }

        // sync message
        let read_req = Message::create_lseq_read_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, read_req);
        self.rx.recv().unwrap();
        self.barrier.wait();

        // now testing
        let read_req = Message::create_lseq_read_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, read_req);
        let response = self.rx.recv().unwrap();
        match response {
            Message::LSeqReadResponse(m) => {
                if self.output {
                    println!("{}", m.result)
                }
            },
            _ => panic!(),
        };

        self.barrier.wait();
        self.running
            .swap(false, std::sync::atomic::Ordering::SeqCst);
    }
}
