extern crate xcopen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let root = env::current_dir().expect("an access to a current directory");
    let entries = xcopen::entries(&root);
    if entries.len() == 0 {
        println!("No xcworkspace/xcodeproj file found under current directory");
    } else if entries.len() == 1 {
        open(&entries[0]);
    } else if entries.len() == 2 {
        let first = &entries[0];
        let second = &entries[1];
        if xcopen::is_xcodeproj(&first) && xcopen::is_xcworkspace(&second) {
            open(&second);
        } else if xcopen::is_xcodeproj(&second) && xcopen::is_xcworkspace(&first) {
            open(&first);
        } else {
            show(&entries);
        }
    } else {
        show(&entries);
    }
}

fn open(path: &Path) {
    use std::process::Command;
    Command::new("open")
        .arg(path)
        .output()
        .expect("open a project");
}

fn show(paths: &[PathBuf]) {
    println!("{:?}", paths);
}
