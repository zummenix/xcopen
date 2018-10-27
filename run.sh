#!/usr/bin/env bash

# Poor man's tests :)

mkdir fixtures
touch fixtures/sample.xcodeproj
touch fixtures/sample.xcworkspace
touch fixtures/example.xcodeproj

mkdir fixtures/my
touch fixtures/my/sample.xcodeproj

cargo install -f --path .
xcopen

rm -rf fixtures
