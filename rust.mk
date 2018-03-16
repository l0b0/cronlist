XML_EXTENSIONS = iml xml

.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	find . -name '*.rs' | xargs rustfmt
	cargo clippy -- -D warnings

.PHONY: clean
clean:
	$(RM) -r Cargo.lock target/

include make-includes/variables.mk make-includes/xml.mk
