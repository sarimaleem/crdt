use std::collections::HashSet;
use serde_json::{Value};

pub enum OpType {
    INSERT,
    DELETE,
    ASSIGN,
}

pub struct ClockId {
    node_id: u32,
    clock: u32,
}

pub struct Op {
    id: ClockId,
    deps: HashSet<ClockId>,
    cursor: Vec<String>,
    action: OpType,
    action_value: Option<Value>,
}