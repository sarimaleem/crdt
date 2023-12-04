use crate::network::Network;
use rand::{distributions::Alphanumeric, Rng};
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, mpsc::Receiver};
use std::sync::{Arc, Barrier};

use crate::message::{self, *};
use crate::traits::Runnable;

pub struct SetsClient {
    id: String,
    n_requests: usize,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
    barrier: Arc<Barrier>,
    workload_source: Vec<String>,
}

impl SetsClient {
    pub fn new(
        id: String,
        n_requests: usize,
        network: Network,
        assigned_replica_id: String,
        rx: Receiver<Message>,
        running: Arc<AtomicBool>,
        barrier: Arc<Barrier>,
        workload_source: Vec<String>,
    ) -> Self {
        Self {
            id,
            n_requests,
            network,
            assigned_replica_id,
            rx,
            running,
            barrier,
            workload_source,
        }
    }

    fn handle_sets_get_result(&mut self, r: SetGetResult) {
        println!("{}: {:?}", self.id, r.result);
    }
}

impl Runnable for SetsClient {
    fn run(&mut self) {
        //   for op in &self.operations {
        //       self.network.send_message(&self.assigned_replica_id, op.clone());
        //   }

        let mut rng1 = rand::thread_rng();
        let mut rng2 = rand::thread_rng();

        for _ in 0..self.n_requests {
            let is_insertion = rng1.gen_bool(0.7);
            let idx = rng2.gen_range(0..self.workload_source.len());

            let payload_string = self.workload_source.get(idx).unwrap();

            let message = match is_insertion {
                true => Message::create_set_insert_request(self.id.clone(), payload_string.clone()),
                false => {
                    Message::create_set_remove_request(self.id.clone(), payload_string.clone())
                }
            };

            self.network
                .send_message(&self.assigned_replica_id, message);
        }

        self.barrier.wait();

        let message = Message::create_set_get_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, message);
        self.rx.recv().unwrap();
        self.barrier.wait();

        let message = Message::create_set_get_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, message);
        let response = self.rx.recv().unwrap();
        match response {
            Message::SetGetResult(result) => self.handle_sets_get_result(result),
            _ => panic!(),
        }

        // wait again then kill replicas
        self.barrier.wait();
        self.running
            .swap(false, std::sync::atomic::Ordering::SeqCst);
    }
}
