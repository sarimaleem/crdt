use std::collections::HashSet;
use serde_json::{Value};

pub enum OpType {
    INSERT,
    DELETE,
    ASSIGN,
}

pub struct ClockId {
    pub node_id: u32,
    pub clock: u32,
}

pub struct Op {
    pub id: ClockId,
    pub deps: HashSet<ClockId>,
    pub cursor: Vec<String>,
    pub action: OpType,
    pub action_value: Option<Value>,
}