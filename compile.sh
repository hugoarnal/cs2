#!/usr/bin/env bash
## This script is used to compile when installing or updating cs2.

INSTALL_PATH=/usr/local/bin

cargo build --release

if sudo cp -f $(dirname "$0")/target/release/cs2 $INSTALL_PATH/cs2; then
    echo "Successfully compiled cs2!"
    if [[ $PATH != *"$INSTALL_PATH"* ]]; then
        echo "$INSTALL_PATH is not in your PATH environnement variable."
        echo "Make sure to add it before you can use cs2."
    fi
else
    echo "An error occurred, couldn't compile cs2."
fi
