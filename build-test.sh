cd ./benchmark
cargo run

cd ../with-serde/
cargo clean
time RUSTFLAGS="-A warnings" cargo build

cd ../with-nanoserde/
cargo clean
time RUSTFLAGS="-A warnings" cargo build
