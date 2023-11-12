# CRDT final project

1. Implement framework and command line arguemnts
2. Framework can work with unreliable and reliable messages
3. Figure out consistency stuff with these CRDT's
4. Implement CRDT:
    1. Base impelmentation to compare to (not crdt)
    2. G-only set
    3. PN Set
    4. Last Write Wins Set
    6. OR-Set
    7. Sequence CRDT
5. what are we trying to measure?
    1. Performance
        1. Based on number of clients
        2. Based on number of replicas
        3. Based on number of messages
        4. Randomized network delay
    3. Talk about complexity of each data type

Arguments:
num_clients
num_replicas
num_requests
crdt_type
send_reliability
verbosity?
