use std::collections::HashMap;

use crate::array_replica::VPtr;

// Counter Messages
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

pub struct ArrayInsertRequest {
    pub sender_id: String,
    pub index: i32,
    pub value: char,
}

pub struct ArrayRemoveRequest {
    pub sender_id: String,
    pub index: i32,
}

pub struct ArrayInsertOperation {
    pub at: VPtr,
    pub value: char,
}

pub struct ArrayRemoveOperation {
    pub at: VPtr,
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
        Message::CounterReadResult(CounterReadResult {
            sender_id,
            total_counter,
        })
    }

    pub fn create_counter_merge(sender_id: String, counters: HashMap<String, i32>) -> Message {
        Message::CounterMerge(CounterMerge {
            sender_id,
            counters,
        })
    }
}
