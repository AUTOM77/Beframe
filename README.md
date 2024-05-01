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
. "$HOME/.cargo/env"
rustup update nightly && rustup default nightly
rustup component add rust-std-x86_64-unknown-linux-musl
cargo build --release
./target/release/bf /data
```

### Optimized Time

1. Raw `Processing time: 14857.700856959s` for 200 pq files
2. Rayon `Processing time: 7361.057489454` for 200 pq files
2. Rayon with par_chunks(30) `Processing time: 6945.763954117s` for 200 pq files
2. Rayon with par_chunks(50) `Processing time: 7036.46832694ss` for 200 pq files