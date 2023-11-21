cd ./native
sh ./build.sh
cd ../ts
rm -rf ./node_modules
yarn install
yarn run test