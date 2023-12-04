use crate::message::{Message, CounterReadRequest, CounterMerge, CounterIncrementRequest, SetGetRequest, SetInsertRequest, SetRemoveRequest, SetMerge, CounterReadResult};
use crate::network::Network;
use crate::traits::Runnable;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::mpsc::Receiver;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::cmp;
use clap::builder::Str;
// use crate::replica::VClockCompareResult::LESS_THAN;

enum VClockCompareResult {
    EQUAL,
    LESS_THAN,
    GREATER_THAN,
    CONCURRENT,
}

pub struct VClock {
    pub clock: HashMap<String, i32>,
}

impl VClock {
    fn new(
        total_replicas: i32
    ) -> Self {
        let mut tmp = Self {
            clock: HashMap::new()
        };
        for i in 0..total_replicas {
          tmp.clock.insert(format!("replica_{}", i), 0);
        }
        tmp
    }

    fn new_with_clock(m: HashMap<String, i32>) -> Self {
        Self {
            clock: m
        }
    }


    pub fn compare(&self, rhs: &VClock) -> VClockCompareResult {
        let mut less = false;
        let mut more = false;
        for (replica_id, clock_value) in &self.clock {
            let lhs_stamp = clock_value;
            let rhs_stamp = rhs.clock.get(replica_id).unwrap();
            if lhs_stamp < rhs_stamp {
                less = true
            } else if lhs_stamp > rhs_stamp {
                more = true
            }
        }

        if less & more {
            return VClockCompareResult::CONCURRENT;
        }

        if less {
            return VClockCompareResult::LESS_THAN;
        }

        if more {
            return VClockCompareResult::GREATER_THAN;
        }

        VClockCompareResult::EQUAL
    }

    pub fn merge(clk1: &VClock, clk2: &VClock) -> VClock {
        let mut new_clock: HashMap<String, i32> = HashMap::new();
        for key in clk1.keys() {
            new_clock.insert(key.clone(),
                             cmp::max(
                                 clk1.get(key).unwrap().clone(),
                                 clk2.get(key).unwrap().clone(),
                             ));
        }
        VClock::new_with_clock(new_clock)
    }

    pub fn increment(clk: &VClock, id: &String) -> VClock {
        // let temp = VClock::new_with_clock(self.clock.clone());
        let temp_map = clk.clock.clone();
        let mut temp = VClock::new_with_clock(temp_map);
        // temp.increment(id);
        temp.clock.insert(id.clone(), temp.clock.get(id).unwrap() + 1);
        temp
    }
}

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

    fn handle_increment_request(&mut self, message: CounterIncrementRequest) {
        let current_value = *self.counters.get(&self.id).unwrap_or(&0);
        self.counters.insert(self.id.to_string(), current_value + 1);

        let merge_request = Message::create_counter_merge(self.id.clone(), self.counters.clone());
        self.network.broadcast_replicas(merge_request);
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
                    Message::CounterReadResult(message) => panic!(),
                    _ => panic!(),
                }
            }
        }
    }
}

pub struct SetsReplica {
    id: String,
    total_replicas: i32,
    rx: Receiver<Message>,
    network: Network,
    running: Arc<AtomicBool>,
    adds: HashMap<String, VClock>,
    removes: HashMap<String, VClock>,
}

impl SetsReplica {
    pub fn new(
        id: String,
        total_replicas: i32,
        rx: Receiver<Message>,
        network: Network,
        running: Arc<AtomicBool>,
        adds: HashMap<String, VClock>,
        removes: HashMap<String, VClock>,
    ) -> Self {
        Self {
            id,
            id_prefix,
            total_replicas,
            rx,
            network,
            running,
            adds: HashMap::new(),
            removes: HashMap::new(),
        }
    }

    fn handle_set_get(&mut self, msg: SetGetRequest) {
        let mut result: HashSet<String> = HashSet::new();

        for (item, clk) in &self.removes {
            // let otherClk = self.removes.get(item).unwrap().clock;
            match self.adds.get(&*item) {
                Some(add) => {
                    match add.compare(clk) {
                        LESS_THAN => { result.insert(item.clone()); }
                        _ => {}
                    };
                }
                None => {}
            }
        }

        self.network.send_message(&msg.sender_id, Message::create_set_get_result(self.id.clone(), result));
    }

    fn handle_set_insert(&mut self, message: SetInsertRequest) {
        let string_to_add = message.request;
        let add_clk = self.adds.get(&string_to_add);
        let mut remove_clk = self.removes.get(&string_to_add);

        match (add_clk, remove_clk) {
            (Some(&v), _) | (_, Some(&v)) => {
                let mut clk = VClock::increment(&v, &self.id);
                self.adds.insert(string_to_add, clk);
                self.removes.remove(&string_to_add);
            }
            (_, _) => {
                let mut clk = VClock::new();
                clk.increment(&self.id);
                self.adds.insert(string_to_add, clk);
            }
        }
    }

    fn handle_set_remove(&mut self, message: SetRemoveRequest) {
        let string_to_add = message.request;
        let add_clk = self.adds.get(&string_to_add);
        let mut remove_clk = self.removes.get(&string_to_add);

        match (add_clk, remove_clk) {
            (Some(&v), _) | (_, Some(&v)) => {
                let mut clk = VClock::increment(&v, &self.id);
                self.adds.remove(&string_to_add);
                self.removes.insert(string_to_add, VClock::inc(&string_to_add, clk));
            },
            (_, _) => {
                let mut clk = VClock::new();
                clk.increment(&self.id);
                self.removes.insert(string_to_add, clk);
            },
        }
    }

    fn handle_set_merge(&mut self, message: SetMerge) {}
}

impl Runnable for SetsReplica {
    fn run(&mut self) {
        // todo!()
        while self.running.load(Ordering::SeqCst) {
            let r = self.rx.try_recv();
            if let Ok(message) = r {
                match message {
                    Message::SetGetRequest(message) => self.handle_set_get(message),
                    Message::SetRemoveRequest(message) => self.handle_set_remove(message),
                    Message::SetInsertRequest(message) => self.handle_set_insert(message),
                    Message::SetMerge(message) => self.handle_set_merge(message),
                    _ => panic!(),
                }
            }
        }
    }
}
