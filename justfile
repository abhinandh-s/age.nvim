install:
  echo "Building just.nvim from source..."
  cargo build --release --target-dir ./target
  mkdir -p lua
  mv target/release/libjust.so lua/just.so
