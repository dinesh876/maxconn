server:
	cargo run --bin maxconn-server -- -l 60000 -d

client: 
	cargo run --bin maxconn-client -- -c 10000 -i 10
