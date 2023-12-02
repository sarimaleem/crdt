use crate::message::CounterReadResult;
use crate::network::Network;
use std::sync::mpsc::Receiver;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::{message::Message, traits::Runnable};

pub struct Client {
    id: String,
    n_requests: i32,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
}

impl Client {
    pub fn new(
        id: String,
        n_requests: i32,
        network: Network,
        assigned_replica_id: String,
        rx: Receiver<Message>,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            id,
            n_requests,
            network,
            assigned_replica_id,
            rx,
            running,
        }
    }

    fn handle_counter_read_result(&mut self, r: CounterReadResult) {
        println!("{}: {}", self.id, r.total_counter);
    }
}

impl Runnable for Client {
    // add code here
    //
    fn run(&mut self) {
        for _ in 0..self.n_requests {
            let message = Message::create_counter_increment_request(self.id.clone());
            self.network
                .send_message(&self.assigned_replica_id, message);
        }

        // TODO figure out timeouts and dropped messages here, do we resend? maybe there should be
        // another strategy and a resend on a timeout. maybe there needs to be timestamps on no
        // ack
        thread::sleep(Duration::from_millis(10));
        let message = Message::create_counter_read_request(self.id.clone());
        self.network
            .send_message(&self.assigned_replica_id, message);
        thread::sleep(Duration::from_millis(10));

        while self.running.load(Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    Message::CounterReadResult(result) => self.handle_counter_read_result(result),
                    _ => panic!(),
                }
            }
        }
    }
}
