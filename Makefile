SOURCE_FILES := $(shell find src -type f -name '*.rs')

counter: build
	./target/debug/crdt -c 5 -r 2 -n 5 -t 0

lseq:
	RUST_BACKTRACE=1 ./target/debug/crdt -c 10 -r 10 -n 100 -t 1

set: build
	RUST_BACKTRACE=full ./target/debug/crdt -c 3 -r 2 -n 5 -t 2

build: $(SOURCE_FILES)
	cargo build
