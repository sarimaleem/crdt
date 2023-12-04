use crate::message::{CounterReadResult, SetGetResult};
use crate::network::Network;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::Barrier;
use std::sync::{
    atomic::AtomicBool,
    Arc,
};

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
    ) -> Self {
        Self {
            id,
            n_requests,
            network,
            assigned_replica_id,
            rx,
            running,
            barrier,
        }
    }

    fn handle_counter_read_result(&mut self, r: CounterReadResult) {
        println!("{}: {}", self.id, r.total_counter);
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

pub struct SetsClient {
    id: String,
    n_requests: i32,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
    operations: Vec<Message>,
}

impl SetsClient {
    pub fn new(
        id: String,
        n_requests: i32,
        network: Network,
        assigned_replica_id: String,
        rx: Receiver<Message>,
        running: Arc<AtomicBool>,
        operations: Vec<Message>,
    ) -> Self {
        Self {
            id,
            n_requests,
            network,
            assigned_replica_id,
            rx,
            running,
            operations,
        }
    }

    fn handle_sets_read_result(&mut self, r: SetGetResult) {
        println!("{}: {:?}", self.id, r.result);
    }
}

impl Runnable for SetsClient {
    fn run(&mut self) {
        for op in &self.operations {
            self.network.send_message(&self.assigned_replica_id, op.clone());
        }

        // TODO figure out timeouts and dropped messages here, do we resend? maybe there should be
        // another strategy and a resend on a timeout. maybe there needs to be timestamps on no
        // ack
        // thread::sleep(Duration::from_millis(10));
        // let message = Message::create_counter_read_request(self.id.clone());
        // self.network
        //     .send_message(&self.assigned_replica_id, message);
        // thread::sleep(Duration::from_millis(10));

        while self.running.load(Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    Message::SetGetResult(result) => self.handle_sets_read_result(result),
                    _ => panic!(),
                }
            }
        }
    }
}
