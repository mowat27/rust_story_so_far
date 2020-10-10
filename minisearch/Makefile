SHELL := bash

.PHONY: examples release benchmark

examples: 
	mkdir -p examples
	wget -O examples/words.txt https://raw.githubusercontent.com/dwyl/english-words/master/words.txt
	wget -O examples/pride_and_prejudice.txt https://gutenberg.org/files/1342/1342-0.txt

release:
	cargo build --release

benchmark: release
	@echo fgrep...
	@time fgrep love examples/pride_and_prejudice.txt > /dev/null
	
	@echo python...
	time ./minisearch.py love examples/pride_and_prejudice.txt > /dev/null

	@echo rust...
	time target/release/minisearch love examples/pride_and_prejudice.txt > /dev/null
