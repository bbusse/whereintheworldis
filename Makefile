BIN=target/release/wintwi

all: build strip

build:
	cargo build --release

strip:
	strip $(BIN)

clean:
	cargo clean

.PHONY: all build strip clean
