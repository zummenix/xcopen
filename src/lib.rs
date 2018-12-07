use itertools::Itertools;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

const SPECIAL_DIRS: &[&str] = &["Pods", "node_modules"];

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Decision {
    Open(PathBuf),
    Show(Vec<(PathBuf, Vec<PathBuf>)>),
    NoEntries,
}

pub fn decision(root: &Path) -> Decision {
    let iter = WalkDir::new(root).into_iter().filter_map(|e| e.ok());
    decision_internal(root, iter)
}

fn grouped(entries: Vec<PathBuf>) -> Vec<(PathBuf, Vec<PathBuf>)> {
    entries
        .into_iter()
        .group_by(|entry| entry.parent().unwrap().to_owned())
        .into_iter()
        .map(|(key, group)| (key, group.collect()))
        .collect()
}

fn decision_internal<I, F>(root: &Path, entries_iter: I) -> Decision
where
    I: Iterator<Item = F>,
    F: Entry,
{
    let entries = entries_internal(root, entries_iter);
    if entries.is_empty() {
        Decision::NoEntries
    } else if entries.len() == 1 {
        Decision::Open(entries[0].to_owned())
    } else {
        let groups = grouped(entries);
        if groups.len() == 1 && groups[0].1.len() == 2 {
            let first = groups[0].1[0].to_owned();
            let second = groups[0].1[1].to_owned();
            if is_xcodeproj(&first) && is_xcworkspace(&second) {
                Decision::Open(second)
            } else if is_xcodeproj(&second) && is_xcworkspace(&first) {
                Decision::Open(first)
            } else {
                Decision::Show(groups)
            }
        } else {
            Decision::Show(groups)
        }
    }
}

fn entries_internal<I, F>(root: &Path, entries_iter: I) -> Vec<PathBuf>
where
    I: Iterator<Item = F>,
    F: Entry,
{
    let root_is_special = SPECIAL_DIRS.iter().any(|dir| has_parent(root, dir));
    entries_iter
        .filter(|entry| is_xcodeproj(entry.path()) || is_xcworkspace(entry.path()))
        .filter(|entry| {
            // Skip workspaces under xcodeproj, example:
            // /Backgrounder/Backgrounder.xcodeproj
            // /Backgrounder/Backgrounder.xcodeproj/project.xcworkspace
            // /Backgrounder/Backgrounder.xcworkspace
            let path = entry.path();
            !(is_xcworkspace(path) && path.parent().map_or(false, is_xcodeproj))
        })
        .filter_map(|entry| {
            // Skip any paths that contain a "special dir" iff a root path doesn't contain it.
            let path = entry.path();
            if !root_is_special && SPECIAL_DIRS.iter().any(|dir| has_parent(&path, dir)) {
                None
            } else {
                Some(path.to_owned())
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

trait Entry {
    fn path(&self) -> &Path;
}

impl Entry for DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::expect;
    use expectest::prelude::*;

    impl Entry for PathBuf {
        fn path(&self) -> &Path {
            self.as_path()
        }
    }

    #[test]
    fn filters_out_project_and_workspace_files() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/examples"),
            PathBuf::from("/projects/my/some.txt"),
            PathBuf::from("/projects/my/App.xcodeproj"),
            PathBuf::from("/projects/my/App.xcodeproj/project.xcworkspace"),
            PathBuf::from("/projects/my/App.xcworkspace"),
        ];
        let result = vec![
            PathBuf::from("/projects/my/App.xcodeproj"),
            PathBuf::from("/projects/my/App.xcworkspace"),
        ];
        expect!(entries_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn excludes_pods_directory() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Pods/Pods.xcodeproj"),
            PathBuf::from("/projects/my/App.xcworkspace"),
        ];
        let result = vec![PathBuf::from("/projects/my/App.xcworkspace")];
        expect!(entries_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn includes_pods_directory() {
        let root = PathBuf::from("/projects/my/Pods");
        let input = vec![
            PathBuf::from("/projects/my/Pods/Pods.xcodeproj"),
            PathBuf::from("/projects/my/Pods/some.txt"),
        ];
        let result = vec![PathBuf::from("/projects/my/Pods/Pods.xcodeproj")];
        expect!(entries_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn excludes_node_modules_directory() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/node_modules/react-native/Libraries/Sample.xcodeproj"),
            PathBuf::from("/projects/my/node_modules/react-native/Libraries/RCTLinking.xcodeproj"),
            PathBuf::from("/projects/my/App.xcworkspace"),
        ];
        let result = vec![PathBuf::from("/projects/my/App.xcworkspace")];
        expect!(entries_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn includes_node_modules_directory() {
        let root = PathBuf::from("/projects/my/node_modules");
        let input = vec![
            PathBuf::from("/projects/my/node_modules/react-native/Libraries/Sample.xcodeproj"),
            PathBuf::from("/projects/my/node_modules/react-native/Libraries/RCTLinking.xcodeproj"),
            PathBuf::from("/projects/my/node_modules/some.txt"),
        ];
        let result = vec![
            PathBuf::from("/projects/my/node_modules/react-native/Libraries/Sample.xcodeproj"),
            PathBuf::from("/projects/my/node_modules/react-native/Libraries/RCTLinking.xcodeproj"),
        ];
        expect!(entries_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn groups_entries_by_parent_directory() {
        let input = vec![
            PathBuf::from("/projects/my/one/file1"),
            PathBuf::from("/projects/my/one/file2"),
            PathBuf::from("/projects/my/file0"),
        ];
        let result = vec![
            (
                PathBuf::from("/projects/my/one"),
                vec![
                    PathBuf::from("/projects/my/one/file1"),
                    PathBuf::from("/projects/my/one/file2"),
                ],
            ),
            (
                PathBuf::from("/projects/my"),
                vec![PathBuf::from("/projects/my/file0")],
            ),
        ];
        expect!(grouped(input)).to(be_equal_to(result));
    }

    #[test]
    fn decision_no_entries_if_without_projects() {
        let root = PathBuf::from("/projects/my");
        let input = vec![PathBuf::from("/projects/my/file1")];
        let result = Decision::NoEntries;
        expect!(decision_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn decision_open_one_project() {
        let root = PathBuf::from("/projects/my");
        let input = vec![PathBuf::from("/projects/my/Sample.xcodeproj")];
        let result = Decision::Open(PathBuf::from("/projects/my/Sample.xcodeproj"));
        expect!(decision_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn decision_open_workspace_with_project_first() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcodeproj"),
            PathBuf::from("/projects/my/Sample.xcworkspace"),
        ];
        let result = Decision::Open(PathBuf::from("/projects/my/Sample.xcworkspace"));
        expect!(decision_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn decision_open_workspace_with_project_second() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcworkspace"),
            PathBuf::from("/projects/my/Sample.xcodeproj"),
        ];
        let result = Decision::Open(PathBuf::from("/projects/my/Sample.xcworkspace"));
        expect!(decision_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn decision_show_one_group_three_projects() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcworkspace"),
            PathBuf::from("/projects/my/Sample.xcodeproj"),
            PathBuf::from("/projects/my/Example.xcodeproj"),
        ];
        let result = Decision::Show(vec![(PathBuf::from("/projects/my"), input.clone())]);
        expect!(decision_internal(&root, input.into_iter())).to(be_equal_to(result));
    }

    #[test]
    fn decision_show_multiple_groups() {
        let root = PathBuf::from("/projects/my");
        let input = vec![
            PathBuf::from("/projects/my/Sample.xcworkspace"),
            PathBuf::from("/projects/my/Sample.xcodeproj"),
            PathBuf::from("/projects/my/example/Example.xcodeproj"),
        ];
        let result = Decision::Show(vec![
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
        ]);
        expect!(decision_internal(&root, input.into_iter())).to(be_equal_to(result));
    }
}
