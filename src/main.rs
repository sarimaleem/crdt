mod argoptions;
mod client;
mod replica;
mod traits;

use std::thread::{self, JoinHandle};

use crate::client::Client;
use crate::replica::Replica;
use crate::traits::Runnable;

fn main() {
    let opts = argoptions::ArgOptions::new();
    run(opts);
}

fn create_clients(n_clients: i32) -> Vec<Client> {
    let mut clients = Vec::new();

    for _ in 0..n_clients {
        clients.push(Client::new())
    }

    return clients;
}

fn create_replicas(n_replicas: i32) -> Vec<Replica> {
    let mut replicas = Vec::new();

    for _ in 0..n_replicas {
        replicas.push(Replica::new());
    }

    return replicas;
}

fn run_clients(clients: Vec<Client>) -> Vec<JoinHandle<()>> {
    let mut handles = Vec::new();
    for client in clients {
        let handle = thread::spawn(move || {
            client.run();
        });
        handles.push(handle);
    };

    return handles
}

fn run_replicas(replicas: Vec<Replica>) -> Vec<JoinHandle<()>> {
    let mut handles = Vec::new();
    for replica in replicas {
        let handle = thread::spawn(move || {
            replica.run();
        });
        handles.push(handle);
    }

    return handles;
}

fn run(options: argoptions::ArgOptions) {
    let clients = create_clients(options.num_clients);
    let replicas = create_replicas(options.num_replicas);
    let client_handles = run_clients(clients);
    let replica_handles = run_replicas(replicas);

    for handle in client_handles {
        handle.join().unwrap();
    }

    for handle in replica_handles {
        handle.join().unwrap();
    }
    // wait for them to finish based on some criteria
}
