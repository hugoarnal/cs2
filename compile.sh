#!/usr/bin/env bash
## This script is used to compile when installing or updating cs2.

set -e
set -u

BASE_DIR=$(dirname "$0")

## needed for patches
if [[ ! -d "/usr/local/share/cs2/cs2" ]]; then
    sudo mkdir -p /usr/local/share/cs2
    sudo cp -r $BASE_DIR /usr/local/share/cs2/cs2
fi

INSTALL_PATH=/usr/local/bin

cd $BASE_DIR
cargo build --release

if sudo cp -f $BASE_DIR/target/release/cs2 $INSTALL_PATH/cs2; then
    echo "Successfully compiled cs2!"
    if [[ $PATH != *"$INSTALL_PATH"* ]]; then
        echo "$INSTALL_PATH is not in your PATH environnement variable."
        echo "Make sure to add it before you can use cs2."
    fi
    echo "You can now install epiclang or banana with \"cs2 install\""
else
    echo "An error occurred, couldn't compile cs2."
fi

set +e
set +u
