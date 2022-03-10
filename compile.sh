#!/bin/bash

cargo run
clang-13 test -o executable
chmod +x executable
./executable