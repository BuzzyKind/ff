@echo off
cargo build --release
cargo install --path .



set "TARGET=.\target\release\frog.exe"
setx frog_ff "%TARGET%"
echo Environment variable is set to %TARGET%, reload your terminal