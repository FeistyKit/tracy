.PHONY: all
all: tracy

tracy: $(shell find src/)
	cargo build --release && cp target/release/tracy ./tracy

.PHONY: develop
develop: tracy-devel

tracy-devel: $(shell find src/)
	cargo build && cp target/debug/tracy ./tracy-devel
