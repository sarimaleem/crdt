use crate::message::MessageType;
use crate::network::Network;
use std::sync::mpsc::Receiver;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{message::Message, traits::Runnable};

pub struct Client {
    id: i32,
    n_requests: i32,
    network: Network,
    assigned_replica_id: String,
    rx: Receiver<Message>,
    running: Arc<AtomicBool>,
}

impl Client {
    pub fn new(
        id: i32,
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

    fn handle_add_ok(&mut self, message: Message) {
        // TODO
    }

    fn handle_read_ok(&mut self, message: Message) {
        // TODO
    }
}

impl Runnable for Client {
    // add code here
    fn run(&mut self) {
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
