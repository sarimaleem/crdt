use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct CounterReadRequest {
    pub sender_id: String,
}

#[derive(Clone)]
pub struct CounterIncrementRequest {
    pub sender_id: String,
}

#[derive(Clone)]
pub struct CounterReadResult {
    pub sender_id: String,
    pub total_counter: i32,
}

#[derive(Clone)]
pub struct CounterMerge {
    pub sender_id: String,
    pub counters: HashMap<String, i32>,
}

#[derive(Clone)]
pub struct SetGetRequest {
    pub sender_id: String,
}

#[derive(Clone)]
pub struct SetGetResult {
    pub sender_id: String,
    pub result: HashSet<String>,
}

#[derive(Clone)]
pub struct SetRemoveRequest {
    pub sender_id: String,
    pub request: String,
}

#[derive(Clone)]
pub struct SetMerge {
    pub sender_id: String,
    pub counters: HashMap<String, i32>,
}


#[derive(Clone)]
pub enum Message {
    CounterReadRequest(CounterReadRequest),
    CounterIncrementRequest(CounterIncrementRequest),
    CounterReadResult(CounterReadResult),
    CounterMerge(CounterMerge),
}

impl Message {
    pub fn create_counter_read_request(sender_id: String) -> Message {
        Message::CounterReadRequest(CounterReadRequest { sender_id })
    }

    pub fn create_counter_increment_request(sender_id: String) -> Message {
        Message::CounterIncrementRequest(CounterIncrementRequest { sender_id })
    }

    pub fn create_counter_read_result(sender_id: String, total_counter: i32) -> Message {
        Message::CounterReadResult(CounterReadResult { sender_id, total_counter })
    }

    pub fn create_counter_merge(sender_id: String, counters: HashMap<String, i32>) -> Message {
        Message::CounterMerge(CounterMerge { sender_id, counters })
    }
}
