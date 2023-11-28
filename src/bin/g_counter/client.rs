use crate::message::MessageType;
use crate::network::Network;
use std::collections::HashMap;
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

    fn handle_add_ok(&mut self, _message: Message) {
        // println!("add ok");
        // TODO do something with message? idk honestly
    }

    fn handle_read_ok(&mut self, message: Message) {
        println!("{} total: {}", self.id, message.total_counter);
        // TODO
    }
}

impl Runnable for Client {
    // add code here
    //
    fn run(&mut self) {

        for _ in 0..self.n_requests {
            let message = Message { mtype: MessageType::ADD, sender_id: self.id.clone(), total_counter: -1, counters: HashMap::new()};
            self.network.send_message(&self.assigned_replica_id, message);
        }

        // TODO figure out timeouts and dropped messages here, do we resend? maybe there should be
        // another strategy and a resend on a timeout. maybe there needs to be timestamps on no
        // ack
        thread::sleep(Duration::from_millis(10));
        let message = Message { mtype: MessageType::READ, sender_id: self.id.clone(), total_counter: -1, counters: HashMap::new()};
        self.network.send_message(&self.assigned_replica_id, message);
        thread::sleep(Duration::from_millis(10));

        while self.running.load(Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message.mtype {
                    MessageType::READOK => self.handle_read_ok(message),
                    MessageType::ADDOK => self.handle_add_ok(message),
                    _ => panic!("We should not be here"),
                }
            }
        }
    }
}
