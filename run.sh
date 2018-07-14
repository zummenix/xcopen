#!/usr/bin/env bash

mkdir fixups
touch fixups/sample.xcodeproj
touch fixups/sample.xcworkspace
touch fixups/example.xcodeproj

mkdir fixups/my
touch fixups/my/sample.xcodeproj

cargo install -f --path .
xcopen

rm -rf fixups
