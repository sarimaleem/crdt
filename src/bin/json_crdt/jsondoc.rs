/// a struct that would store the local doc.
use serde_json::{ Value};
use std::{fmt, thread};
use std::sync::Arc;
use tokio::sync::broadcast::{Sender, Receiver};
use crate::op::Op;

#[derive(Debug)]
/// This is struct that would hold the actual json document,
pub struct JSONDoc {
    /// the lamport time of this JSONDoc
    clock: Arc<u32>,
    /// the id of the doc
    pub id: u32,
    /// Doc representation using serde_json
    pub doc: Arc<Value>,
    // sender: Arc<Sender<Op>>,
    // receiver: Arc<Receiver<Op>>,
}

impl JSONDoc {
    pub fn new(self, id: u32, sender: &mut Sender<Op>) -> JSONDoc {
        JSONDoc {
            id: (id),
            clock: (Arc::new(0)),
            doc: Arc::new(Value::Object(Default::default())),
            // sender: Arc::new(sender.clone()),
            // receiver: Arc::new(self.sender.subscribe()),
        }
    }

    pub fn apply_op(&mut self, operation: Op) {

    }


    pub fn protocol(&mut self, operations: Vec<Op>, tx_ref: &Sender<Op>) {
        let tx = Arc::new(tx_ref.clone());
        let mut rx = Arc::new(tx.subscribe());
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::spawn(async move {
                    loop {
                        match rx.recv().await {
                            Ok(op) =>  {self.apply_op(op);},
                            _Err => {break;}
                        }
                    }
                });

                for operation in operations {
                    tokio::spawn(async move {
                        tx.send(operation).unwrap();
                    });
                }

            });
        drop(tx);
    }
}

impl fmt::Display for JSONDoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the Display trait
        write!(f, "{}", self.doc)
    }
}