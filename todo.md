### g-counter
- [o] Make a framework that we can use
    - [X] Add a trait for clients and a trait for replicas
    - [X] Do Argument Parsing (James)
    - [ ] Make a main method that runs all the data types
    - [X] Implement messages
    - [ ] Add an ending condition
- [ ] Implement some sort of correctness checker
    - [ ] Brainstorm ideas of how that would work
    - [ ] Maybe based on logs?
- [ ] Implement CRDT
    - [ ] Base implementation to compare to (not crdt)
    - [ ] G-only set
    - [ ] PN Set
    - [ ] Last Write Wins Set
    - [ ] OR-Set
    - [ ] Sequence CRDT
- [ ] Implement Performance Benchmarking
    - [ ] Try with different number of clients/requests
    - [ ] Try with send unreliability, see convergence times with/without unreliability
    - [ ] See how fast it converges
- [ ] Write report
    - [ ] Compare complexity of each implementation
    - [ ] Compare performance of each implementation


### json-crdt
- state representation
    - local state
        - how to build and modify jsons
    - op struct
        - should just pass them in channels