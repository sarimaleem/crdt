pub struct Message {
    mtype: MessageType,
    timestamp: i32,
    tag: i32,
    counters: Vector<i32>
}

pub enum MessageType {
    READ,
    READ_OK,
    ADD,
    ADD_OK,
}

