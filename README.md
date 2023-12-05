# CRDT Implementations in Rust

This repository contains implementations of three conflict-free replicated data types (CRDTs) using Rust:

1. **GCounter**: A simple distributed counter supporting increment and read operations.
2. **Set**: A distributed set allowing for insert, remove, and read operations.
3. **LSeq**: An ordered sequence (similar to an array) supporting insert, remove, and read operations.

## Prerequisites

Before running the code, ensure you have the following installed:
- Rust programming language
- Cargo (Rust's package manager and build tool)

## Installation

Clone the repository to your local machine:

```bash
git clone [repository URL]
cd [repository directory]
```

## Building the Code

To build the project, use the following command:

```bash
make build
```

This command compiles the source files located in the `src` directory.

## Running the CRDT Implementations

The project supports different CRDT types, which can be executed with specific commands. Each type can be configured with various options such as the number of clients, replicas, and requests.

### Running the GCounter (Counter)

To run the GCounter, execute the following command:

```bash
make counter
```

This runs the counter CRDT with 5 clients, 2 replicas, and 5 requests.

### Running the LSeq

To run the LSeq, use this command:

```bash
make lseq
```

This runs the LSeq CRDT with 10 clients, 10 replicas, and 100 requests, with full Rust backtrace enabled.

### Running the Set

To run the Set, use:

```bash
make set
```

This runs the set CRDT with 3 clients, 2 replicas, and 5 requests, and a full Rust backtrace.

## Custom Configuration

You can also run the CRDTs with custom configurations using the following format:

```bash
./target/debug/crdt -c [num_clients] -r [num_replicas] -n [num_requests] -t [crdt_type] -p [send_reliability] -k [check]
```

- `num_clients`: Number of clients (default: 1)
- `num_replicas`: Number of replicas (default: 2)
- `num_requests`: Number of requests (default: 5)
- `crdt_type`: Type of CRDT (0 for Counter, 1 for LSeq, 2 for Set)
- `send_reliability`: Probability of successful message delivery (default: 1.0)
- `check`: Boolean flag for additional checks (default: false)
