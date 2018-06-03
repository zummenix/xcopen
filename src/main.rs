extern crate xcopen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let root = env::current_dir().expect("an access to a current directory");
    let files = xcopen::files(&root);
    if files.len() == 0 {
        println!("No projects found");
    } else if files.len() == 1 {
        open(&files[0]);
    } else if files.len() == 2 {
        let first = &files[0];
        let second = &files[1];
        if xcopen::is_xcodeproj(&first) && xcopen::is_xcworkspace(&second) {
            open(&second);
        } else if xcopen::is_xcodeproj(&second) && xcopen::is_xcworkspace(&first) {
            open(&first);
        } else {
            show(&files);
        }
    } else {
        show(&files);
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
