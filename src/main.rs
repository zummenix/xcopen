extern crate xcopen;

use xcopen::Decision;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let root = env::current_dir().expect("an access to a current directory");
    match xcopen::decision(&root) {
        Decision::NoEntries => {
            println!("No xcworkspace/xcodeproj file found under current directory");
        }
        Decision::Open(path) => open(&path),
        Decision::Show(groups) => println!("{:?}", groups),
    }
}

fn open(path: &Path) {
    use std::process::Command;
    Command::new("open")
        .arg(path)
        .output()
        .expect("open a project");
}
