# xcopen

A convenient way to open xcodeproj/xcworkspace from a command line.

## Features

- knows about `xcodeproj`/`xcworkspace` files and prefers `xcworkspace` by default
- skips searching projects under `Pods` or `node_modules` directories unless runs inside them
- asks which project to open if there are more under a current directory

## Installation

The tool is written in Rust, so first you need to make sure that you have the compiler installed.
If you don't have it, head to https://rustup.rs and follow instructions.

If you have Rust installed, run:

```bash
cargo install --git "https://github.com/zummenix/xcopen"
```

## Usage

Just run `xcopen` in a directory with projects.

## LICENSE

MIT
