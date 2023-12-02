- [X] Make a framework that we can use
    - [X] Add a trait for clients and a trait for replicas
    - [X] Do Argument Parsing (James)
    - [X] Make a main method that runs all the data types
    - [X] Implement messages
    - [X] Add an ending condition
- [.] Implement CRDT
    - [X] G-only set
    - [ ] PN Set
    - [ ] Last Write Wins Set
    - [ ] OR-Set
    - [ ] Sequence CRDT
    - [ ] Spreadsheet
- [ ] Implement Performance Benchmarking
    - [ ] Try with different number of clients/requests
    - [ ] Try with send unreliability, see convergence times with/without unreliability
    - [ ] See how fast it converges
- [ ] Write report
    - [ ] Compare complexity of each implementation
    - [ ] Compare performance of each implementation


enum Message {
    GCOUNTER_READ
    GCOUNTER_ADD
    GCOUNTER_MERGE
    LIST_READ
    LIST_ADD
}

struct StructA {
    field_a: i32,
}

struct StructB {
    field_b: String,
}

enum MyData {
    A(StructA),
    B(StructB),
}

1. Messages generic (30 mins - 1 hr)
2. Change the main method to make it generic (30 mins - 1hr)
3. Implement differnt clients and replicas
5. Performance Benchmarking
6. Write paper

