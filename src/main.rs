use xcopen::DirStatus;

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

const SPECIAL_DIRS: &[&str] = &["Pods", "node_modules", ".build", "Carthage"];

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Opt {
    /// A directory where to start search for project files
    #[structopt(parse(from_os_str))]
    dir: Option<PathBuf>,
}

fn main() -> Result<(), main_error::MainError> {
    let opt = Opt::from_args();
    let dir = opt.dir.unwrap_or(env::current_dir()?);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let entries_iter = WalkDir::new(&dir)
        .into_iter()
        .filter_map(Result::ok)
        .map(|dir_entry| dir_entry.into_path());
    match xcopen::dir_status(&dir, entries_iter, &SPECIAL_DIRS) {
        DirStatus::NoEntries => {
            Err(format!("No project files found under {}", dir.to_string_lossy()).into())
        }
        DirStatus::Project(path) => open(&path),
        DirStatus::Groups(groups) => {
            let mut number: u32 = 1;
            let mut projects_map: HashMap<u32, PathBuf> = HashMap::new();
            let mut sorted_groups = groups.into_iter().collect::<Vec<_>>();
            sorted_groups.sort_by(|a, b| a.0.cmp(&b.0));
            for (group, mut projects) in sorted_groups {
                writeln!(&mut stdout, "in {}:", group.to_string_lossy())?;
                projects.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
                for project in projects {
                    if let Some(file_name) = project.file_name() {
                        writeln!(
                            &mut stdout,
                            "    {}. {}",
                            number,
                            file_name.to_string_lossy()
                        )?;
                        projects_map.insert(number, project);
                        number += 1;
                    }
                }
            }
            let mut rl = rustyline::Editor::<()>::new();
            let line = rl.readline("Enter the number of the project to open: ")?;
            line.parse::<u32>()
                .ok()
                .and_then(|number| projects_map.get(&number))
                .map_or(Ok(()), open)
        }
    }
}

/// Tries to open xcworkspace/xcodeproj file using `open` tool.
fn open(path: impl AsRef<Path>) -> Result<(), main_error::MainError> {
    use std::process::Command;
    Command::new("open").arg(path.as_ref()).spawn()?.wait()?;
    Ok(())
}
