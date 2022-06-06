.PHONY: all
all: tracy

tracy: $(shell find src/)
	cargo build --release && cp target/release/tracy ./tracy
