#!/bin/bash

APP_NAME="Ball Physics Sim"
RUST_CRATE_NAME="three_dee"
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"
cp Info.plist "${APP_NAME}.app/Contents/Info.plist"
cp AppIcon.icns "${APP_NAME}.app/Contents/Resources/AppIcon.icns"
cp -a assets "${APP_NAME}.app/Contents/MacOS/"
cargo build --release --target x86_64-apple-darwin 
cargo build --release --target aarch64-apple-darwin 

lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
