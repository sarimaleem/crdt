use crate::message::{Message, CounterReadRequest, CounterMerge, CounterIncrementRequest};
use crate::network::Network;
use crate::traits::Runnable;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
pub struct Replica {
    id: String,
    rx: Receiver<Message>,
    network: Network,
    running: Arc<AtomicBool>,
    counters: HashMap<String, i32>,
}

impl Replica {
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
            counters: HashMap::new(),
        }
    }

    fn handle_read(&mut self, message: CounterReadRequest) {
        let sum: i32 = self.counters.values().sum();
        let new_message = Message::create_counter_read_result(self.id.clone(), sum);
        self.network.send_message(&message.sender_id, new_message);
    }

    fn handle_increment_request(&mut self, _message: CounterIncrementRequest) {
        let current_value = *self.counters.get(&self.id).unwrap_or(&0);
        self.counters.insert(self.id.to_string(), current_value + 1);

        let merge_request = Message::create_counter_merge(self.id.clone(), self.counters.clone());
        self.network.broadcast_replicas(merge_request);
        
        // TODO: maybe ack the message?
    }

    fn handle_merge(&mut self, message: CounterMerge) {
        for (node, counter) in message.counters.iter() {
            self.counters.insert(node.to_string(), std::cmp::max(*counter, *self.counters.get(node).unwrap_or(&0)));
        }
    }
}

impl Runnable for Replica {
    fn run(&mut self) {
        while self.running.load(Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    Message::CounterReadRequest(message) => self.handle_read(message),
                    Message::CounterIncrementRequest(message) => self.handle_increment_request(message),
                    Message::CounterMerge(message) => self.handle_merge(message),
                    _ => panic!()
                }
            }
        }
    }
}
