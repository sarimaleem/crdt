use clap::{Arg,App};

pub enum CrdtTypes {
    G_COUNTER,
    PN_COUNTER,
    // blah blah blah add then if you wish
}

pub struct ArgOptions {
    pub num_clients: i32,
    pub num_replcias: i32,
    pub num_requests: i32,
    pub crdt_type: CrdtTypes,
    pub send_reliability: f64,
    pub verbosity: i32,
}

impl ArgOptions {
    pub fn new() -> ArgOptions {
        let default_num_clients = "1";
        let default_num_replcias = "2";
        let default_num_requests = "10";
        let default_crdt_type = "0";
        let default_send_reliability = "1";
        let default_verbosity = "0";

        let matches = App::new("CRDTs")
            .version("0.0.1")
            .author("Sarim Aleem & Sizhan Xu")
            .about("CRDTs written in Rust")
            .arg()
            .arg()
            .arg()
            .arg()
            .arg()
            .arg()
            .get_matches();

        let num_clients = matches.value_of()
    }
}