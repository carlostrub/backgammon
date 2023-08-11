# Make readme creates a README.md file from the src/lib.rs file's documentation.
# The README.md file is used as the crate's README on crates.io.
# 
# Usage: make readme
#

all: readme doc

.PHONY: readme
readme:
	cargo readme > README.md

.PHONY: doc
doc:
	cargo doc
	rm -r /usr/local/www/backgammon/doc/backgammon;
	cp -r target/doc/backgammon /usr/local/www/backgammon/doc/backgammon;

doc-all:
	rm -r /usr/local/www/backgammon/doc;
	cp -r target/doc /usr/local/www/backgammon/doc;
