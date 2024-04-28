# Beframe

Turn video into frames async

```sh
ffmpeg -i 1.mp4  -ss 00:00:01 -frames:v -o 1.png
ffmpeg -i 1.mp4  -ss 00:00:01 -frames:v -o 1.jpg
```
### Compile from source

```bash
sudo apt install -y --no-install-recommends pkg-config yasm nasm musl-dev clang llvm

curl -fsSL https://sh.rustup.rs | sh -s -- -y
rustup update nightly && rustup default nightly
rustup component add rust-std-x86_64-unknown-linux-musl
cargo build --release
```