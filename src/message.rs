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

#[derive(Clone)]
pub struct ArrayInsertRequest {
    pub sender_id: String,
    pub index: usize,
    pub value: char,
}

#[derive(Clone)]
pub struct ArrayRemoveRequest {
    pub sender_id: String,
    pub index: usize,
}

#[derive(Clone)]
pub struct ArrayInsertOperation {
    pub sender_id: String,
    pub at: VPtr,
    pub value: char,
}

#[derive(Clone)]
pub struct ArrayRemoveOperation {
    pub sender_id: String,
    pub at: VPtr,
}

#[derive(Clone)]
pub enum Message {
    CounterReadRequest(CounterReadRequest),
    CounterIncrementRequest(CounterIncrementRequest),
    CounterReadResult(CounterReadResult),
    CounterMerge(CounterMerge),
    ArrayInsertRequest(ArrayInsertRequest),
    ArrayInsertOperation(ArrayInsertOperation),
    ArrayRemoveRequest(ArrayRemoveRequest),
    ArrayRemoveOperation(ArrayRemoveOperation),
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

    pub fn create_array_insert_request(sender_id: String, index: usize, value: char) -> Message {
        Message::ArrayInsertRequest(ArrayInsertRequest {
            sender_id,
            index,
            value,
        })
    }

    pub fn create_array_remove_request(sender_id: String, index: usize) -> Message {
        Message::ArrayRemoveRequest(ArrayRemoveRequest { sender_id, index })
    }

    pub fn create_array_insert_operation(sender_id: String, at: VPtr, value: char) -> Message {
        Message::ArrayInsertOperation(ArrayInsertOperation {
            sender_id,
            at,
            value,
        })
    }

    pub fn create_array_remove_operation(sender_id: String, at: VPtr) -> Message {
        Message::ArrayRemoveOperation(ArrayRemoveOperation { sender_id, at })
    }
}
