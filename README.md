# Double Finder

Find duplicate files in a directory. Recursively traverses the directory and adds all files whose checksums match to a database.

## Installation

To install probably do something like

```sh
cargo build --release
ln -s $(pwd)/target/release/double-finder /usr/local/bin/double-finder
```

## Usage

```sh
$ cargo run --release -q -- .
[...]
found  23 doubles: ./target/rls/debug/incremental/double_finder-1wi75gwof358j/s-g4860fc93a-17d6thd.lock
                   ./target/rls/debug/incremental/double_finder-s5tc62vhs5yn/s-g4860f70pf-rxgn1y.lock
                   ./target/rls/debug/.cargo-lock
                   ./target/rls/debug/deps/libdouble_finder-0c2ff78c1288bded.rmeta
                   ./target/rls/debug/deps/libdouble_finder-edb7dab3835a5d77.rmeta
                   ./target/rls/debug/build/generic-array-7df5835d0e47e7c2/stderr
                   ./target/rls/debug/build/typenum-dc228ee2490b4610/stderr
                   ./target/rls/debug/build/libc-51b64e39aa78f10f/stderr
                   ./target/release/.cargo-lock
                   ./target/release/build/typenum-26a24ce130106053/stderr
                   ./target/release/build/libc-cc0b87ad3556d2e8/stderr
                   ./target/release/build/generic-array-4dc84bd357f32a91/stderr
                   ./target/debug/incremental/double_finder-3th0dbuw1jub8/s-g485zl012j-5nfgd7.lock
                   ./target/debug/incremental/double_finder-1wi75gwof358j/s-g4860f95xm-1m1winr.lock
                   ./target/debug/incremental/double_finder-29bo0hh1z4h3u/s-g486py072w-vb1yd3.lock
                   ./target/debug/incremental/double_finder-s5tc62vhs5yn/s-g4860f8aj4-7xz2rp.lock
                   ./target/debug/.cargo-lock
                   ./target/debug/deps/libdouble_finder-0c2ff78c1288bded.rmeta
                   ./target/debug/deps/libdouble_finder-7e5e525c8ff13437.rmeta
                   ./target/debug/deps/libdouble_finder-edb7dab3835a5d77.rmeta
                   ./target/debug/build/generic-array-7df5835d0e47e7c2/stderr
                   ./target/debug/build/typenum-dc228ee2490b4610/stderr
                   ./target/debug/build/libc-51b64e39aa78f10f/stderr
[...]
found 593/953 doubles
```
