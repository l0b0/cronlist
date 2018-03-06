.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	find . -name '*.rs' | xargs rustfmt

.PHONY: clean
clean:
	$(RM) -r Cargo.lock target/

include make-includes/variables.mk
