mod argoptions;
mod counter;
mod lseq;
mod message;
mod network;
mod traits;

use crate::traits::Runnable;
use argoptions::ArgOptions;
use counter::client::CounterClient;
use counter::replica::CounterReplica;
use message::Message;
use network::Network;

use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, Barrier};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let opts = ArgOptions::new();
    run(opts);
}

fn create_counter_nodes(args: ArgOptions, barrier: Arc<Barrier>, running: Arc<AtomicBool>) -> Vec<Box<dyn Runnable + Send>> {
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
        let replica = CounterReplica::new(
            format!("replica_{}", i),
            replica_receivers.remove(0),
            network.clone(),
            running.clone(),
        );
        nodes.push(Box::new(replica));
    }

    // create clients
    for i in 0..args.num_clients {
        let assigned_replica = (i % args.num_replicas) as usize;
        let assigned_replica_id = format!("replica_{}", assigned_replica);
        let client = CounterClient::new(
            format!("client_{}", i),
            args.num_requests,
            network.clone(),
            assigned_replica_id,
            client_receivers.remove(0),
            running.clone(),
            barrier.clone()
        );
        nodes.push(Box::new(client));
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
