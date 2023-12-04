use std::collections::{HashMap, HashSet};

use crate::set::vclock::VClock;
use crate::lseq::vptr::VPtr;

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
pub struct LSeqInsertRequest {
    pub sender_id: String,
    pub index: usize,
    pub value: char,
}

#[derive(Clone)]
pub struct LSeqRemoveRequest {
    pub sender_id: String,
    pub index: usize,
}

#[derive(Clone)]
pub struct LSeqInsertOperation {
    pub sender_id: String,
    pub at: VPtr,
    pub value: char,
}

#[derive(Clone)]
pub struct LSeqRemoveOperation {
    pub sender_id: String,
    pub at: VPtr,
}

#[derive(Clone)]
pub struct LSeqReadRequest {
    pub sender_id: String,
}

#[derive(Clone)]
pub struct LSeqReadResponse {
    pub sender_id: String,
    pub result: String,
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
    LSeqReadRequest(LSeqReadRequest),
    LSeqInsertRequest(LSeqInsertRequest),
    LSeqInsertOperation(LSeqInsertOperation),
    LSeqRemoveRequest(LSeqRemoveRequest),
    LSeqRemoveOperation(LSeqRemoveOperation),
    LSeqReadResponse(LSeqReadResponse),
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

    pub fn create_lseq_insert_request(sender_id: String, index: usize, value: char) -> Message {
        Message::LSeqInsertRequest(LSeqInsertRequest {
            sender_id,
            index,
            value,
        })
    }

    pub fn create_lseq_remove_request(sender_id: String, index: usize) -> Message {
        Message::LSeqRemoveRequest(LSeqRemoveRequest { sender_id, index })
    }

    pub fn create_lseq_read_request(sender_id: String) -> Message {
        Message::LSeqReadRequest(LSeqReadRequest { sender_id })
    }

    pub fn create_lseq_read_response(sender_id: String, result: String) -> Message {
        Message::LSeqReadResponse(LSeqReadResponse { sender_id, result })
    }

    pub fn create_lseq_insert_operation(sender_id: String, at: VPtr, value: char) -> Message {
        Message::LSeqInsertOperation(LSeqInsertOperation {
            sender_id,
            at,
            value,
        })
    }

    pub fn create_lseq_remove_operation(sender_id: String, at: VPtr) -> Message {
        Message::LSeqRemoveOperation(LSeqRemoveOperation { sender_id, at })
    }

    pub fn create_set_get_result(sender_id: String,  result: HashSet<String>) -> Message{
        Message::SetGetResult(SetGetResult {sender_id, result})
    }

    pub fn create_set_merge(sender_id: String, adds: HashMap<String, VClock>, removes: HashMap<String, VClock>) -> Message {
        Message::SetMerge(SetMerge { sender_id, add_map:adds, remove_map: removes})
    }
}
