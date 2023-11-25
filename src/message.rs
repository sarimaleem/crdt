use std::collections::HashMap;

pub struct Message {
    pub mtype: MessageType,
    pub sender_id: String,
    // TODO change the way this is done later to make it more generic
    pub total_counter: i32,
    pub counters: HashMap<String, i32>,
}

#[derive(Copy, Clone)]
pub enum MessageType {
    READ,
    ADD,
    MERGE,
    READOK,
    ADDOK,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self {
            mtype: self.mtype,
            sender_id: self.sender_id.clone(),
            total_counter: self.total_counter,
            counters: self.counters.clone(),
        }
    }
}

impl Message {
    pub fn new(
        mtype: MessageType,
        sender_id: String,
        total_counter: i32,
        counters: HashMap<String, i32>,
    ) -> Self {
        return Self {
            mtype,
            sender_id,
            total_counter,
            counters,
        };
    }
}
