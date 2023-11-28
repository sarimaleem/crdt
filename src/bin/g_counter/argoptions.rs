use clap::{arg, command, value_parser};

pub enum CrdtTypes {
    GCounter,
    PnCounter,
    // blah blah blah add them if you wish
}

pub struct ArgOptions {
    pub num_clients: i32,
    pub num_replicas: i32,
    pub num_requests: i32,
    pub crdt_type: CrdtTypes,
    pub send_reliability: f64,
    pub verbosity: i32,
}

impl ArgOptions {
    pub fn new() -> ArgOptions {
        let matches = command!()
            .arg(
                arg!(-c[num_clients])
                    .required(false)
                    .value_parser(value_parser!(i32))
                    .default_value("1"),
            )
            .arg(
                arg!(-r[num_replicas])
                    .required(false)
                    .value_parser(value_parser!(i32))
                    .default_value("2"),
            )
            .arg(
                arg!(-n[num_requests])
                    .required(false)
                    .value_parser(value_parser!(i32))
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
                arg!(-v[verbosity])
                    .required(false)
                    .value_parser(value_parser!(i32))
                    .default_value("0"),
            )
            .get_matches();

        let num_clients = matches.get_one::<i32>("num_clients").unwrap().clone();
        let num_replicas = matches.get_one::<i32>("num_replicas").unwrap().clone();
        let num_requests = matches.get_one::<i32>("num_requests").unwrap().clone();
        let crdt_type: CrdtTypes = match matches.get_one::<i32>("crdt_type").unwrap() {
            0 => CrdtTypes::GCounter,
            1 => CrdtTypes::PnCounter,
            _ => panic!(),
        };
        let send_reliability = matches.get_one::<f64>("send_reliability").unwrap().clone();
        let verbosity = matches.get_one::<i32>("verbosity").unwrap().clone();

        ArgOptions {
            num_clients,
            num_replicas,
            num_requests,
            crdt_type,
            send_reliability,
            verbosity,
        }
    }
}
