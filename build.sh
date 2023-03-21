#!/bin/sh

# temp build script

INSTALL_ROOT=""

cargo build --release --target x86_64-unknown-linux-gnu

sudo install -Dm755 target/x86_64-unknown-linux-gnu/release/matsud "${INSTALL_ROOT}/usr/bin/matsud"
sudo install -Dm755 target/x86_64-unknown-linux-gnu/release/matsucli "${INSTALL_ROOT}/usr/bin/matsucli"
sudo install -Dm644 matsuba_default.toml "${INSTALL_ROOT}/usr/share/matsuba/matsuba_default.toml"
sudo install -Dm644 services/matsuba.service "${INSTALL_ROOT}/usr/lib/systemd/user/matsuba.service"

