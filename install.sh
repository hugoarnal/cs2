#!/usr/bin/env bash

REPO_LINK="https://github.com/hugoarnal/cs2.git"
DEFAULT_BASE_DIR="/usr/local/share/cs2"

read -p "Specify installation path [default: $DEFAULT_BASE_DIR]: " BASE_DIR
BASE_DIR=${BASE_DIR:-$DEFAULT_BASE_DIR}

mkdir -p $BASE_DIR
git clone $REPO_LINK $BASE_DIR/cs2

$BASE_DIR/cs2/compile.sh
