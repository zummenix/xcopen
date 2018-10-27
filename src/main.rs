extern crate rustyline;
extern crate structopt;
extern crate xcopen;

use xcopen::Decision;

use std::alloc::System;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[global_allocator]
static A: System = System;

#[derive(Debug, StructOpt)]
struct Opt {
    /// A directory where to search for project files
    #[structopt(parse(from_os_str))]
    root: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let root = opt.root.unwrap_or(env::current_dir()?);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    match xcopen::decision(&root) {
        Decision::NoEntries => writeln!(
            &mut handle,
            "No xcworkspace/xcodeproj file found under '{}'",
            root.to_string_lossy()
        ),
        Decision::Open(path) => open(&path),
        Decision::Show(groups) => {
            let mut number: u32 = 1;
            let mut map: HashMap<u32, PathBuf> = HashMap::new();
            for (group, projects) in groups {
                writeln!(&mut handle, "in {}:", group.to_string_lossy())?;
                for project in projects {
                    if let Some(file_name) = project.file_name() {
                        writeln!(
                            &mut handle,
                            "   {}. {}",
                            number,
                            file_name.to_string_lossy()
                        )?;
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
                            return open(&project);
                        }
                    }
                    Ok(())
                }
                Err(_) => Ok(()),
            }
        }
    }
}

fn open(path: &Path) -> io::Result<()> {
    use std::process::Command;
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let output = Command::new("open").arg(path).output()?;
    write!(handle, "{}", String::from_utf8_lossy(&output.stderr))
}
