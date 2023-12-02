use crate::message::Message;
use crate::network::Network;
use crate::traits::Runnable;
use std::sync::mpsc::Receiver;
use std::sync::{atomic::AtomicBool, Arc};

use std::cmp::Ordering;

pub struct VPtr {
    sequence: Vec<u8>,
    id: String,
}

impl Ord for VPtr {
    fn cmp(&self, other: &Self) -> Ordering {
        let len = std::cmp::min(self.sequence.len(), other.sequence.len());
        for i in 0..len {
            let cmp = self.sequence[i].cmp(&other.sequence[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        if self.sequence.len() != other.sequence.len() {
            return self.sequence.len().cmp(&other.sequence.len());
        }

        self.id.cmp(&other.id)
    }
}

impl PartialOrd for VPtr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for VPtr {}

impl PartialEq for VPtr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn generate_seq(lo: &[u8], hi: &[u8]) -> Vec<u8> {
    let mut acc: Vec<u8> = Vec::new();
    let mut i = 0;

    loop {
        let min = if i >= lo.len() { 0 } else { lo[i] };
        let max = if i >= hi.len() { 255 } else { hi[i] };

        if min + 1 < max {
            acc.push(min + 1);
            return acc;
        } else {
            acc.push(min);
        }

        i += 1;
    }
}

pub struct ArrayReplica {
    id: String,
    rx: Receiver<Message>,
    network: Network,
    running: Arc<AtomicBool>,
    array: Vec<VPtr>,
}

impl ArrayReplica {}

impl Runnable for ArrayReplica {
    fn run(&mut self) {
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    _ => panic!(),
                }
            }
        }
    }
}
