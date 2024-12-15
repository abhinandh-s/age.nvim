install:
  echo "Building age.nvim from source..."
  cargo build --release --target-dir ./target
  mkdir -p lua
  mv target/release/libage.so lua/age.so
