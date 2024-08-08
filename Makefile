CARGO = cargo

all: build

build:
	$(CARGO) build --release

run:
	./target/release/ctn

test:
	$(CARGO) test

clean:
	$(CARGO) clean

install:
	$(CARGO) install --path .

uninstall:
	cargo uninstall ctn

help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "  all      - Build the project (default)"
	@echo "  build    - Build the project in release mode"
	@echo "  run      - Run the release version of the project"
	@echo "  test     - Run tests"
	@echo "  clean    - Clean the project"
	@echo "  install  - Install the binary to your cargo bin path"
	@echo "  uninstall - Uninstall the binary from your cargo bin path"