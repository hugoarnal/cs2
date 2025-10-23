#!/usr/bin/env bash

set -e
set -u

REPO_LINK="https://github.com/hugoarnal/cs2.git"
DEFAULT_BASE_DIR="/usr/local/share/cs2"
TMP_DIR="/tmp/cs2-cs2"

# read -p "Specify installation path [default: $DEFAULT_BASE_DIR]: " BASE_DIR
BASE_DIR=${BASE_DIR:-$DEFAULT_BASE_DIR}

if [ -d $BASE_DIR ]; then
    echo "cs2 seems to already be installed at $BASE_DIR"
    echo "Try running cs2 update instead or removing the directory at $BASE_DIR"
    exit 1
fi

sudo mkdir -p $BASE_DIR

if [ ! -d $TMP_DIR ]; then
    git clone $REPO_LINK $TMP_DIR
else
    echo "$TMP_DIR is already installed, continuing..."
fi

$TMP_DIR/compile.sh

## move cs2 installed repo to $BASE_DIR/cs2
sudo mv $TMP_DIR $BASE_DIR/cs2
sudo chown -R $USER $BASE_DIR/cs2

set +e
set +u
