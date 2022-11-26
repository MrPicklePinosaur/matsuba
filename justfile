
default: matsud

matsucli:
    RUST_LOG=info cargo run --bin matsucli

matsud:
    RUST_LOG=info cargo run --bin matsud

build:
    cargo build

check:
    cargo check

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

