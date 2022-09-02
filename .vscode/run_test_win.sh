setx CARGO_INCREMENTAL "0"
setx RUSTFLAGS "-Cinstrument-coverage"
setx LLVM_PROFILE_FILE "cargo-test-%p-%m.profdata"
cargo test
