use crate::message::Message;
use crate::message::MessageType;
use crate::network::Network;
use crate::traits::Runnable;
use std::sync::mpsc::Receiver;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct Replica {
    id: i32,
    rx: Receiver<Message>,
    network: Network,
    running: Arc<AtomicBool>,
}

impl Replica {
    pub fn new(
        id: i32,
        rx: Receiver<Message>,
        network: Network,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            id,
            rx,
            network,
            running,
        }
    }

    fn handle_read(&mut self, message: Message) {
        println!("TODO");
        // send with network.send
    }

    fn handle_add(&mut self, message: Message) {
        println!("TODO");
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
                    _ => panic!("We should not be here"),
                }
            }
        }
    }
}
