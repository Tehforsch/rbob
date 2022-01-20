cross build --release --target x86_64-unknown-linux-gnu
if [[ $? == 0 ]]; then
    cp target/x86_64-unknown-linux-gnu/release/bob $supermucProjects/bob/
fi
