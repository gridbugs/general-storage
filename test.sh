#!/bin/bash

set -euxo pipefail

export RUSTFLAGS="--deny warnings"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cargo clean

cargo test --workspace
cargo test --manifest-path storage/Cargo.toml --all-features
cargo test --manifest-path file/Cargo.toml --all-features
cargo test --manifest-path web/Cargo.toml --all-features

cargo clean

find -name node_modules -exec rm -rf {} \; || true
find -name package-lock.json -delete || true
find -name dist -exec rm -rf {} \; || true

pushd $DIR/examples/fib/web
npm install
npm run build -- --mode development
popd
