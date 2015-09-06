CARGO=cargo

all: c0c

c0c: bin/c0c
bin/c0c: release
	mkdir -p bin
	cp target/release/l1-compiler bin/c0c

release:
	$(CARGO) build --release

debug:
	$(CARGO) build

clean:
	$(CARGO) clean
	rm -f bin/c0c
	rm -f src/parse/lexer_generated.rs
	rm -f src/parse/parser.rs

.PHONY: c0c debug release clean
