extern crate xcopen;

use std::env;
use xcopen::files;

fn main() {
    let root = env::current_dir().expect("an access to a current directory");
    println!("{:?}", root);

    for file in files(&root) {
        println!("{:?}", file);
    }
}
