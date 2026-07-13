CARGO = cargo

all: Cargo.toml
	@$(CARGO) build --release

check: Cargo.toml
	@$(CARGO) test -- --nocapture

clean: Cargo.toml
	@rm -rf *~ target
	@$(CARGO) clean

maintainer-clean: Cargo.toml
	@rm -f Cargo.lock

.PHONY: all check clean maintainer-clean
.SECONDARY:
.SUFFIXES:
