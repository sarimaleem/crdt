use std::collections::{HashMap, HashSet};

use crate::replica::VClock;

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
pub struct SetInsertRequest {
    pub sender_id: String,
    pub request: String,
}

#[derive(Clone)]
pub struct SetRemoveRequest {
    pub sender_id: String,
    pub request: String,
}

#[derive(Clone)]
pub struct SetMerge {
    pub sender_id: String,
    pub add_map: HashMap<String, VClock>,
    pub remove_map: HashMap<String, VClock>,
}


#[derive(Clone)]
pub enum Message {
    CounterReadRequest(CounterReadRequest),
    CounterIncrementRequest(CounterIncrementRequest),
    CounterReadResult(CounterReadResult),
    CounterMerge(CounterMerge),
    SetGetRequest(SetGetRequest),
    SetGetResult(SetGetResult),
    SetInsertRequest(SetInsertRequest),
    SetRemoveRequest(SetRemoveRequest),
    SetMerge(SetMerge),
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

    pub fn create_set_get_result(sender_id: String,  result: HashSet<String>) -> Message{
        Message::SetGetResult(SetGetResult {sender_id, result})
    }

    pub fn create_set_merge(sender_id: String, adds: HashMap<String, VClock>, removes: HashMap<String, VClock>) -> Message {
        Message::SetMerge(SetMerge { sender_id, add_map:adds, remove_map: removes})
    }
}
