#!/bin/bash 
rm -rf pkg
wasm-pack build
cd www
npm uninstall chess_engine 
cp package-old.json package.json
npm install
