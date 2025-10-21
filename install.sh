#!/usr/bin/env bash

set -e
set -u

REPO_LINK="https://github.com/hugoarnal/cs2.git"
DEFAULT_BASE_DIR="/usr/local/share/cs2"

# read -p "Specify installation path [default: $DEFAULT_BASE_DIR]: " BASE_DIR
BASE_DIR=${BASE_DIR:-$DEFAULT_BASE_DIR}

sudo mkdir -p $BASE_DIR
git clone $REPO_LINK /tmp/cs2-cs2
make -C /tmp/cs2-cs2 release
sudo make -C /tmp/cs2-cs2 install
sudo mv /tmp/cs2-cs2 $BASE_DIR/cs2
sudo chown -R $USER $BASE_DIR/cs2

set +e
set +u
