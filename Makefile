server:
	cargo run --bin maxconn-server -- -d

client: 
	cargo run --bin maxconn-client -- -c 100 -i 10 -d