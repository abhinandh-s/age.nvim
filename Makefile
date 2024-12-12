all:
	echo "Building just.nvim from source with mail flag..."
	cargo build --release --features mail --target-dir ./target
	mkdir -p lua
	mv target/release/libjust.so lua/just.so

install:
	echo "Building just.nvim from source..."
	cargo build --release --target-dir ./target
	mkdir -p lua
	mv target/release/libjust.so lua/just.so
