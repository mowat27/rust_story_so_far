SHELL := bash

.PHONY: examples benchmark clean

RUST_RELEASE := target/release/minisearch
DICTIONARY_SENTINAL := examples/dictionary.sentinal
PRIDE_AND_PREJIDICE_SENTINAL := examples/pride_and_prejudice.sentinal

examples/dictionary.txt: $(DICTIONARY_SENTINAL)
examples/pride_and_prejudice.txt: $(PRIDE_AND_PREJIDICE_SENTINAL)

$(DICTIONARY_SENTINAL):
	wget -O examples/words.txt https://raw.githubusercontent.com/dwyl/english-words/master/dictionary.txt
	touch $(DICTIONARY_SENTINAL)

$(PRIDE_AND_PREJIDICE_SENTINAL):	
	wget -O examples/pride_and_prejudice.txt https://gutenberg.org/files/1342/1342-0.txt
	touch $(PRIDE_AND_PREJIDICE_SENTINAL)

$(RUST_RELEASE): src
	cargo build --release

examples: examples/pride_and_prejudice.txt examples/dictionary.txt

benchmark: $(RUST_RELEASE) examples/pride_and_prejudice.txt
	# Warmup caches 
	@fgrep love examples/pride_and_prejudice.txt > /dev/null
	@./minisearch.py love examples/pride_and_prejudice.txt > /dev/null
	@$(RUST_RELEASE) love examples/pride_and_prejudice.txt > /dev/null


	@printf "Using fgrep...\n"
	time fgrep love examples/pride_and_prejudice.txt > /dev/null
	
	@printf "\n\nUsing Python...\n"
	time ./minisearch.py love examples/pride_and_prejudice.txt > /dev/null

	@printf "\n\nUsing Rust...\n"
	time $(RUST_RELEASE) love examples/pride_and_prejudice.txt > /dev/null

clean: 
	cargo clean
	rm -f examples/*.txt examples/*.sentinal