use crate::message::Message;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Network {
    senders: HashMap<String, Sender<Message>>,
    send_prob: f64,
}

impl Network {
    pub fn new() -> Self {
        Self {
            senders: HashMap::new(),
            send_prob: 1.0
        }
    }

    pub fn add_sender(&mut self, id: String, sender: Sender<Message>) {
        self.senders.insert(id, sender);
    }

    pub fn send_message(&mut self, id: &String, message: Message) {
        let sender = self.senders.get(id).expect("Bad Address");
        // TODO add drop probability stuff here
        sender.send(message).unwrap();
    }

    pub fn broadcast_replicas(&mut self, message: Message) {
        for (id, sender) in self.senders.iter() {
            if id.starts_with("replica") {
                sender.send(message.clone()).unwrap();
            }
        }
    }
}

impl Clone for Network {
    fn clone(&self) -> Self {
        Self {
            senders: self.senders.clone(),
            send_prob: self.send_prob
        }
    }
}
