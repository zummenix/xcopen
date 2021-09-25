use itertools::Itertools;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A status of the directory.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DirStatus {
    /// The directory contains one project.
    Project(PathBuf),
    /// The directory contains multiple projects grouped by directories.
    Groups(HashMap<PathBuf, Vec<PathBuf>>),
    /// The directory doesn't have any projects.
    NoEntries,
}

/// Returns a status of the directory.
pub fn dir_status<I>(root: &Path, entries_iter: I, special_dirs: &[&str]) -> DirStatus
where
    I: Iterator<Item = PathBuf>,
{
    let entries = filter_entries(root, entries_iter, special_dirs);
    if entries.is_empty() {
        DirStatus::NoEntries
    } else if entries.len() == 1 {
        DirStatus::Project(entries.into_iter().next().unwrap())
    } else {
        let groups = grouped(entries);
        if groups.len() == 1 {
            match groups.iter().next().unwrap().1.as_slice() {
                [first, second] => match (is_xcworkspace(&first), is_xcworkspace(&second)) {
                    (true, false) => DirStatus::Project(first.to_owned()),
                    (false, true) => DirStatus::Project(second.to_owned()),
                    (_, _) => DirStatus::Groups(groups),
                },
                _ => DirStatus::Groups(groups),
            }
        } else {
            DirStatus::Groups(groups)
        }
    }
}

fn grouped(entries: Vec<PathBuf>) -> HashMap<PathBuf, Vec<PathBuf>> {
    entries
        .into_iter()
        .map(|entry| (entry.parent().unwrap().to_owned(), entry))
        .into_group_map()
}

fn filter_entries<I>(root: &Path, entries_iter: I, special_dirs: &[&str]) -> Vec<PathBuf>
where
    I: Iterator<Item = PathBuf>,
{
    let root_is_special = special_dirs.iter().any(|dir| has_parent(root, dir));
    entries_iter
        .filter(|entry| is_xcodeproj(&entry) || is_xcworkspace(&entry))
        .filter(|entry| {
            // Skip workspaces under xcodeproj, example:
            // /Backgrounder/Backgrounder.xcodeproj
            // /Backgrounder/Backgrounder.xcodeproj/project.xcworkspace
            // /Backgrounder/Backgrounder.xcworkspace
            !(is_xcworkspace(entry) && entry.parent().map_or(false, is_xcodeproj))
        })
        .filter_map(|entry| {
            // Skip any paths that contain a "special dir" iff a root path doesn't contain it.
            if !root_is_special && special_dirs.iter().any(|dir| has_parent(&entry, dir)) {
                None
            } else {
                Some(entry)
            }
        })
        .collect()
}

fn is_xcodeproj(path: &Path) -> bool {
    has_extension(path, "xcodeproj")
}

fn is_xcworkspace(path: &Path) -> bool {
    has_extension(path, "xcworkspace")
}

fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().map_or(false, |ext| ext == extension)
}

fn has_parent(path: &Path, parent: &str) -> bool {
    if path.ends_with(parent) {
        true
    } else {
        match path.parent() {
            Some(path) => has_parent(path, parent),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::expect;
    use expectest::prelude::*;

    #[test]
    fn dir_status_is_no_entries_without_projects() {
        let root = PathBuf::from("/projects/my");
        let input = vec![PathBuf::from("/projects/my/file1")];
        let result = DirStatus::NoEntries;
        expect!(dir_status(&root, input.into_iter(), &[])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_is_open_for_one_project() {
        let root = PathBuf::from("/projects/my");
        let input = vec![PathBuf::from("/projects/my/Sample.xcodeproj")];
        let result = DirStatus::Project(PathBuf::from("/projects/my/Sample.xcodeproj"));
        expect!(dir_status(&root, input.into_iter(), &[])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_is_open_for_one_project_inside_excluded_directory() {
        let root = PathBuf::from("/projects/my/Pods");
        let input = vec![PathBuf::from("/projects/my/Pods/Sample.xcodeproj")];
        let result = DirStatus::Project(PathBuf::from("/projects/my/Pods/Sample.xcodeproj"));
        expect!(dir_status(&root, input.into_iter(), &["Pods"])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_is_no_entries_for_excluded_directory() {
        let root = PathBuf::from("/projects/my");
        let input = vec![PathBuf::from("/projects/my/Pods/Sample.xcodeproj")];
        let result = DirStatus::NoEntries;
        expect!(dir_status(&root, input.into_iter(), &["Pods"])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_is_open_workspace_if_xcodeproj_exists_first() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcodeproj"),
            PathBuf::from("/projects/my/Sample.xcworkspace"),
        ];
        let result = DirStatus::Project(PathBuf::from("/projects/my/Sample.xcworkspace"));
        expect!(dir_status(&root, input.into_iter(), &[])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_open_workspace_if_xcodeproj_exists_second() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcworkspace"),
            PathBuf::from("/projects/my/Sample.xcodeproj"),
        ];
        let result = DirStatus::Project(PathBuf::from("/projects/my/Sample.xcworkspace"));
        expect!(dir_status(&root, input.into_iter(), &[])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_show_one_group_three_projects() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcworkspace"),
            PathBuf::from("/projects/my/Sample.xcodeproj"),
            PathBuf::from("/projects/my/Example.xcodeproj"),
        ];
        let result = DirStatus::Groups(
            vec![(PathBuf::from("/projects/my"), input.clone())]
                .into_iter()
                .collect(),
        );
        expect!(dir_status(&root, input.into_iter(), &[])).to(be_equal_to(result));
    }

    #[test]
    fn dir_status_show_multiple_groups() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcworkspace"),
            PathBuf::from("/projects/my/Sample.xcodeproj"),
            PathBuf::from("/projects/my/example/Example.xcodeproj"),
        ];
        let result = DirStatus::Groups(
            vec![
                (
                    PathBuf::from("/projects/my"),
                    vec![
                        PathBuf::from("/projects/my/Sample.xcworkspace"),
                        PathBuf::from("/projects/my/Sample.xcodeproj"),
                    ],
                ),
                (
                    PathBuf::from("/projects/my/example"),
                    vec![PathBuf::from("/projects/my/example/Example.xcodeproj")],
                ),
            ]
            .into_iter()
            .collect(),
        );
        expect!(dir_status(&root, input.into_iter(), &[])).to(be_equal_to(result));
    }
}
