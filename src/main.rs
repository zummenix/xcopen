extern crate walkdir;
extern crate xcopen;

use std::env;
use walkdir::WalkDir;
use xcopen::is_xcode_file;

fn main() {
    let current_dir = env::current_dir().expect("an access to a current directory");
    println!("{:?}", current_dir);
    for entry in WalkDir::new(current_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_xcode_file)
    {
        println!("{}", entry.path().display());
    }
}
