.PHONY: all count test

all:
	cargo run

count:
	fd | grep -E "\.rs" | xargs wc -l

test:
	cargo test
