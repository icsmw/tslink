#!/bin/sh
set -e

export TSLINK_BUILD=true

cargo clean
rm -rf ./output 2> /dev/null
rm -rf ./dist 2> /dev/null
cargo build
cd ./ts_check
rm -rf ./dist 2> /dev/null
rm -rf ./node_module 2> /dev/null
yarn install
yarn run build