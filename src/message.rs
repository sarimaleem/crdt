pub struct Message {
    pub mtype: MessageType,
    pub sender_id: String,
}

pub enum MessageType {
    READ,
    ADD,
    READOK,
    ADDOK,
}

