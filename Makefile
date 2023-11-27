simple: build
	./target/debug/g-counter -c 5 -r 2 -n 5 -t 0

build: 
	cargo build