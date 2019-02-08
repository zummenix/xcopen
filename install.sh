#!/usr/bin/env bash

cargo install --force --path .

XCOPEN_PATH=$(which xcopen)
echo "Binary size before strip:"
du -h ${XCOPEN_PATH}
strip ${XCOPEN_PATH}
echo "Binary size after strip:"
du -h ${XCOPEN_PATH}
