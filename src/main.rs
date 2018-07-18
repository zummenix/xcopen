extern crate rustyline;
extern crate xcopen;

use xcopen::Decision;

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let root = env::current_dir().expect("an access to a current directory");
    match xcopen::decision(&root) {
        Decision::NoEntries => {
            writeln!(
                &mut handle,
                "No xcworkspace/xcodeproj file found under current directory"
            );
        }
        Decision::Open(path) => open(&path),
        Decision::Show(groups) => {
            let mut number: u32 = 1;
            let mut map: HashMap<u32, PathBuf> = HashMap::new();
            for (group, projects) in groups {
                writeln!(&mut handle, "in {}:", group.to_string_lossy());
                for project in projects {
                    if let Some(file_name) = project.file_name() {
                        writeln!(
                            &mut handle,
                            "   {}. {}",
                            number,
                            file_name.to_string_lossy()
                        );
                        map.insert(number, project.to_owned());
                        number += 1;
                    }
                }
            }
            let mut rl = rustyline::Editor::<()>::new();
            match rl.readline("Enter the number to open: ") {
                Ok(line) => {
                    if let Ok(number) = line.parse::<u32>() {
                        if let Some(project) = map.get(&number) {
                            open(&project);
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}

fn open(path: &Path) {
    use std::process::Command;
    Command::new("open")
        .arg(path)
        .output()
        .expect("open a project");
}
