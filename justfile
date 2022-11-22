
default: out

matsucli:
    cargo run --bin matsucli

matsud:
    cargo run --bin matsud

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

