use clap::{arg, command, value_parser};

pub enum CrdtTypes {
    Counter,
    LSeq,
    Set,
}

pub struct ArgOptions {
    pub num_clients: usize,
    pub num_replicas: usize,
    pub num_requests: usize,
    pub crdt_type: CrdtTypes,
    pub send_reliability: f64,
    pub check: bool,
}

impl ArgOptions {
    pub fn new() -> ArgOptions {
        let matches = command!()
            .arg(
                arg!(-c[num_clients])
                    .required(false)
                    .value_parser(value_parser!(usize))
                    .default_value("1"),
            )
            .arg(
                arg!(-r[num_replicas])
                    .required(false)
                    .value_parser(value_parser!(usize))
                    .default_value("2"),
            )
            .arg(
                arg!(-n[num_requests])
                    .required(false)
                    .value_parser(value_parser!(usize))
                    .default_value("5"),
            )
            .arg(
                arg!(-t[crdt_type])
                    .required(false)
                    .value_parser(value_parser!(i32))
                    .default_value("0"),
            )
            .arg(
                arg!(-p[send_reliability])
                    .required(false)
                    .value_parser(value_parser!(f64))
                    .default_value("1.0"),
            )
            .arg(
                arg!(-k[check])
                    .required(false)
                    .value_parser(value_parser!(bool))
                    .default_value("false"),
            )
            .get_matches();

        let num_clients = matches.get_one::<usize>("num_clients").unwrap().clone();
        let num_replicas = matches.get_one::<usize>("num_replicas").unwrap().clone();
        let num_requests = matches.get_one::<usize>("num_requests").unwrap().clone();
        let crdt_type: CrdtTypes = match matches.get_one::<i32>("crdt_type").unwrap() {
            0 => CrdtTypes::Counter,
            1 => CrdtTypes::LSeq,
            2 => CrdtTypes::Set,
            _ => panic!(),
        };
        let send_reliability = matches.get_one::<f64>("send_reliability").unwrap().clone();
        let check = matches.get_one::<bool>("check").unwrap().clone();

        ArgOptions {
            num_clients,
            num_replicas,
            num_requests,
            crdt_type,
            send_reliability,
            check,
        }
    }
}
