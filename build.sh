#!/bin/bash
set -e

if [ -f "alfred-app-switcher.alfredworkflow" ]; then
    echo "  rm old file"
    rm alfred-app-switcher.alfredworkflow
fi

cargo build --release

cp target/release/app-switcher workflow/

cd workflow

zip -r ../alfred-app-switcher.alfredworkflow *
