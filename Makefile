# Make readme creates a README.md file from the src/lib.rs file's documentation.
# The README.md file is used as the crate's README on crates.io.
# 
# Usage: make readme
#

readme:
	cargo readme > README.md

.PHONY: readme
