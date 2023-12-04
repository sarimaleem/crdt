mod argoptions;
mod counter;
mod lseq;
mod message;
mod network;
mod set;
mod traits;

use crate::traits::Runnable;
use argoptions::ArgOptions;
use counter::client::CounterClient;
use counter::replica::{self, CounterReplica};
use lseq::client::LSeqClient;
use lseq::replica::LSeqReplica;

use message::Message;
use network::Network;
use set::client::SetsClient;
use set::replica::SetsReplica;

use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, Barrier};
use std::thread::{self, JoinHandle};

fn main() {
    let opts = ArgOptions::new();
    run(opts);
}

fn create_counter_nodes(
    args: ArgOptions,
    barrier: Arc<Barrier>,
    running: Arc<AtomicBool>,
) -> Vec<Box<dyn Runnable + Send>> {
    let mut nodes: Vec<Box<dyn Runnable + Send>> = Vec::new();
    let mut network = Network::new();

    // create transmitters for replicas
    let mut replica_receivers = Vec::new();
    let mut client_receivers = Vec::new();

    // create replica channels
    for i in 0..args.num_replicas {
        let (tx, rx) = mpsc::channel::<Message>();
        replica_receivers.push(rx);
        let replica_id = format!("replica_{}", i);
        network.add_sender(replica_id, tx);
    }

    // create client channels
    for i in 0..args.num_clients {
        let (tx, rx) = mpsc::channel::<Message>();
        client_receivers.push(rx);
        let client_id = format!("client_{}", i);
        network.add_sender(client_id, tx);
    }

    // create replicas
    for i in 0..args.num_replicas {
        match args.crdt_type {
            argoptions::CrdtTypes::Counter => {
                let replica = CounterReplica::new(
                    format!("replica_{}", i),
                    replica_receivers.remove(0),
                    network.clone(),
                    running.clone(),
                );
                nodes.push(Box::new(replica));
            }
            argoptions::CrdtTypes::LSeq => {
                let replica = LSeqReplica::new(
                    format!("replica_{}", i),
                    replica_receivers.remove(0),
                    network.clone(),
                    running.clone(),
                );
                nodes.push(Box::new(replica));
            }
            argoptions::CrdtTypes::Set => {
                let replica = SetsReplica::new(
                    format!("replica_{}", i),
                    args.num_replicas,
                    replica_receivers.remove(0),
                    network.clone(),
                    running.clone(),
                );
                nodes.push(Box::new(replica));
            }
        };
    }

    // create clients
    for i in 0..args.num_clients {
        let assigned_replica = (i % args.num_replicas) as usize;
        let assigned_replica_id = format!("replica_{}", assigned_replica);
        match args.crdt_type {
            argoptions::CrdtTypes::Counter => {
                let client = CounterClient::new(
                    format!("client_{}", i),
                    args.num_requests,
                    network.clone(),
                    assigned_replica_id,
                    client_receivers.remove(0),
                    running.clone(),
                    barrier.clone(),
                );
                nodes.push(Box::new(client));
            }
            argoptions::CrdtTypes::LSeq => {
                let client = LSeqClient::new(
                    format!("client_{}", i),
                    args.num_requests,
                    network.clone(),
                    assigned_replica_id,
                    client_receivers.remove(0),
                    running.clone(),
                    barrier.clone(),
                );
                nodes.push(Box::new(client));
            }
            // TODO: 
            argoptions::CrdtTypes::Set => {
                let workload: Vec<String> = vec!{
                    "String1".to_string(),
                    "String2".to_string(),
                    "String3".to_string(),
                    "String4".to_string(),
                    "String5".to_string(),
                    "String6".to_string(),
                    "String7".to_string(),
                    "String8".to_string(),
                    "String9".to_string(),
                    "String10".to_string()
                };
                let client = SetsClient::new(
                    format!("client_{}", i),
                    args.num_requests,
                    network.clone(),
                    assigned_replica_id,
                    client_receivers.remove(0),
                    running.clone(),
                    barrier.clone(),
                    workload
                );
                nodes.push(Box::new(client));
            }
        }
    }

    return nodes;
}

fn run_nodes(nodes: Vec<Box<dyn Runnable + Send>>) -> Vec<JoinHandle<()>> {
    let mut handles = Vec::new();
    for mut node in nodes {
        let handle = thread::spawn(move || {
            node.run();
        });
        handles.push(handle);
    }

    return handles;
}

fn run(options: argoptions::ArgOptions) {
    let running = Arc::new(AtomicBool::new(true));
    let barrier = Arc::new(Barrier::new(options.num_clients));

    let nodes = create_counter_nodes(options, barrier, running.clone());
    let handles = run_nodes(nodes);

    for handle in handles {
        handle.join().unwrap();
    }
}
