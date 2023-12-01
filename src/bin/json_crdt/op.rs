use std::collections::HashSet;
use serde_json::{Value};

#[derive(Debug, Clone)]
pub enum OpType {
    INSERT,
    DELETE,
    ASSIGN,
}

#[derive(Debug, Clone)]
pub struct ClockId {
    pub node_id: u32,
    pub clock: u32,
}

#[derive(Debug, Clone)]
pub struct Op {
    pub id: ClockId,
    pub deps: HashSet<ClockId>,
    pub cursor: Vec<String>,
    pub action: OpType,
    pub action_value: Option<Value>,
}