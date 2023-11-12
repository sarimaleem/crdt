
pub mod argoptions;

fn main() {
    // println!("Hello, world!");
    let opts = argoptions::ArgOptions::new();
    println!("args read!")
}

fn run(options: argoptions::ArgOptions) {
    // create clients
    // create replicas
    // start clients/replicas
    // wait for them to finish based on some criteria
}