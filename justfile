
default: matsud

matsucli:
    cargo run --bin matsucli

matsud:
    cargo run --bin matsud

matsuime:
    cargo run --bin matsuime

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

