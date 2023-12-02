mod argoptions;
mod client;
mod message;
mod replica;
mod traits;
mod network;
mod array_client;
mod array_replica;

use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use argoptions::ArgOptions;
use message::Message;
use network::Network;

use crate::client::Client;
use crate::replica::Replica;
use crate::traits::Runnable;

fn main() {
    let opts = ArgOptions::new();
    run(opts);
}

fn create_nodes(args: ArgOptions, running: Arc<AtomicBool>) -> Vec<Box<dyn Runnable + Send>> {

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
        let replica = Replica::new(
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
        let client = Client::new(
            format!("client_{}", i),
            args.num_requests,
            network.clone(),
            assigned_replica_id,
            client_receivers.remove(0),
            running.clone(),
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
    };

    return handles
}


// FIXME this is a work in progress
fn run(options: argoptions::ArgOptions) {
    let running = Arc::new(AtomicBool::new(true));
    let nodes = create_nodes(options, running.clone());
    let handles = run_nodes(nodes);

    thread::sleep(Duration::from_millis(500));
    running.swap(false, std::sync::atomic::Ordering::SeqCst);

    for handle in handles {
        handle.join().unwrap();
    }

    // TODO wait for them to finish based on some criteria
}
