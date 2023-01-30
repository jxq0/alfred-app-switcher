#!/bin/bash
set -e

aarch_file=alfred-app-switcher-aarch64.alfredworkflow
if [ -f $aarch_file ]; then
    echo "  rm old file"
    rm $aarch_file
fi

echo "  build for aarch64-apple-darwin"
cargo build --release
cp target/release/app-switcher workflow/

pushd workflow
zip -r ../$aarch_file *
popd

echo ""

x86_64_file=alfred-app-switcher-x86_64.alfredworkflow
if [ -f $x86_64_file ]; then
    echo "  rm old file"
    rm $x86_64_file
fi

echo "  build for x86_64-apple-darwin"
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/app-switcher workflow/

pushd workflow
zip -r ../$x86_64_file *
popd
