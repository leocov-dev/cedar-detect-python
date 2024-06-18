#!/usr/bin/env sh

#apt install -y protobuf-compiler
yum install protobuf-compiler

# install rust
curl -sSf https://sh.rustup.rs | sh -s -- -y

rustup update