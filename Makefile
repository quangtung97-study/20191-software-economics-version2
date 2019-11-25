.PHONY: all count

all:
	cargo run

count:
	fd | grep -E "\.rs" | xargs wc -l
