export TSLINK_BUILD=true
cargo test -- --nocapture
cd ./tests/callbacks
yarn run test
cd ../renaming
yarn run test
cd ../node-bindgen
yarn run test
cd ../../examples/node_bindgen
sh ./run_test.sh