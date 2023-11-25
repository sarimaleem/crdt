use crate::message::Message;
use crate::message::MessageType;
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

    fn handle_read(&mut self, message: Message) {
        let sum: i32 = self.counters.values().sum();
        let new_message = Message::new(MessageType::READOK, self.id.clone(), sum, self.counters.clone());
        self.network.send_message(&message.sender_id, new_message);
    }

    fn handle_add(&mut self, message: Message) {
        let current_value = *self.counters.get(&self.id).unwrap_or(&0);
        self.counters.insert(self.id.to_string(), current_value + 1);
        let broadcast_message = Message::new(MessageType::MERGE, self.id.to_string(), -1, self.counters.clone());
        let ack_message = Message::new(MessageType::ADDOK, self.id.to_string(), -1, self.counters.clone());
        self.network.broadcast_replicas(broadcast_message);
        self.network.send_message(&message.sender_id, ack_message);
    }

    fn handle_merge(&mut self, message: Message) {
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
                match message.mtype {
                    MessageType::READ => self.handle_read(message),
                    MessageType::ADD => self.handle_add(message),
                    MessageType::MERGE => self.handle_merge(message),
                    _ => panic!("We should not be here"),
                }
            }
        }
    }
}
