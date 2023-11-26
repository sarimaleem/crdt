simple: build
	./target/debug/crdt -c 5 -r 2 -n 5 -t 0

build: src/*.rs
	cargo build