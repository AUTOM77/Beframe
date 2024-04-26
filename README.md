# Beframe

Turn video into frames async


### Compile from source

```bash
sudo apt install -y --no-install-recommends pkg-config yasm nasm musl-dev clang llvm

curl -fsSL https://sh.rustup.rs | sh -s -- -y
rustup update nightly && rustup default nightly
rustup component add rust-std-x86_64-unknown-linux-musl
cargo build --release
```