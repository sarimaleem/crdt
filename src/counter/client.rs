use crate::message::CounterReadResult;
use crate::network::Network;
use std::sync::mpsc::Receiver;
use std::sync::Barrier;
use std::sync::{atomic::AtomicBool, Arc};

/**
TODO: create a enum to include all the clients
 */
use crate::{message::Message, traits::Runnable};

pub struct CounterClient {
    id: String,
    n_requests: usize,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
    barrier: Arc<Barrier>,
    output: bool,
}

impl CounterClient {
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

    fn handle_counter_read_result(&mut self, r: CounterReadResult) {
        if self.output {
            println!("{}: {}", self.id, r.total_counter);
        }
    }
}

impl Runnable for CounterClient {
    fn run(&mut self) {
        for _ in 0..self.n_requests {
            let message = Message::create_counter_increment_request(self.id.clone());
            self.network
                .send_message(&self.assigned_replica_id, message);
        }

        // sync message ensure all my writes have been received
        let message = Message::create_counter_read_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, message);
        self.rx.recv().unwrap();
        self.barrier.wait();

        // now testing
        let message = Message::create_counter_read_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, message);
        let response = self.rx.recv().unwrap();
        match response {
            Message::CounterReadResult(result) => self.handle_counter_read_result(result),
            _ => panic!(),
        }

        // wait again then kill replicas
        self.barrier.wait();
        self.running
            .swap(false, std::sync::atomic::Ordering::SeqCst);
    }
}
