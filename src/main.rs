use xcopen::DirStatus;

use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// A directory where to search for project files
    #[structopt(parse(from_os_str))]
    root: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let root = opt.root.unwrap_or(env::current_dir()?);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    match xcopen::dir_status(&root) {
        DirStatus::NoEntries => Err(Error::Own(format!(
            "No xcworkspace/xcodeproj file found under '{}'",
            root.to_string_lossy()
        ))),
        DirStatus::Project(path) => open(&path),
        DirStatus::Groups(groups) => {
            let mut number: u32 = 1;
            let mut map: HashMap<u32, PathBuf> = HashMap::new();
            for (group, projects) in groups {
                writeln!(&mut handle, "in {}:", group.to_string_lossy())?;
                for project in projects {
                    if let Some(file_name) = project.file_name() {
                        writeln!(
                            &mut handle,
                            "    {}. {}",
                            number,
                            file_name.to_string_lossy()
                        )?;
                        map.insert(number, project);
                        number += 1;
                    }
                }
            }
            let mut rl = rustyline::Editor::<()>::new();
            let line = rl.readline("Enter the number of the project to open: ")?;
            let number = line.parse::<u32>().unwrap_or(0);
            if let Some(path) = map.get(&number) {
                open(&path)
            } else {
                Ok(())
            }
        }
    }
}

/// Tries to open xcworkspace/xcodeproj file using `open` tool.
fn open(path: &Path) -> Result<(), Error> {
    use std::process::Command;
    let output = Command::new("open").arg(path).output()?;
    if output.stderr.is_empty() {
        Ok(())
    } else {
        Err(Error::Own(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

/// An error of this CLI.
enum Error {
    Io(io::Error),
    Rl(rustyline::error::ReadlineError),
    Own(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(io_error) => io_error.fmt(f),
            Error::Rl(rl_error) => rl_error.fmt(f),
            Error::Own(error) => f.write_str(&error),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement Debug in terms of Display for nice printing in the console.
        fmt::Display::fmt(self, f)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(e: rustyline::error::ReadlineError) -> Error {
        Error::Rl(e)
    }
}
