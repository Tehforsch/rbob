rm target/release/bob
cargo build --release --locked
cp target/release/bob ~/.bin/
